pub mod debug;
pub mod dep_node;
mod edges;
mod graph;
mod query;
mod serialized;

use std::panic;

pub use dep_node::{DepKind, DepKindStruct, DepNode, DepNodeParams, WorkProductId};
pub(crate) use graph::DepGraphData;
pub use graph::{DepGraph, DepNodeIndex, TaskDepsRef, WorkProduct, WorkProductMap, hash_result};
pub use query::DepGraphQuery;
use rustc_data_structures::profiling::SelfProfilerRef;
use rustc_session::Session;
pub use serialized::{SerializedDepGraph, SerializedDepNodeIndex};
use tracing::instrument;

use self::graph::{MarkFrame, print_markframe_trace};
use crate::ich::StableHashingContext;

pub trait DepContext: Copy {
    type Deps: Deps;

    /// Create a hashing context for hashing new results.
    fn with_stable_hashing_context<R>(self, f: impl FnOnce(StableHashingContext<'_>) -> R) -> R;

    /// Access the DepGraph.
    fn dep_graph(&self) -> &DepGraph<Self::Deps>;

    /// Access the profiler.
    fn profiler(&self) -> &SelfProfilerRef;

    /// Access the compiler session.
    fn sess(&self) -> &Session;

    fn dep_kind_info(&self, dep_node: DepKind) -> &DepKindStruct<Self>;

    #[inline(always)]
    fn fingerprint_style(self, kind: DepKind) -> FingerprintStyle {
        let data = self.dep_kind_info(kind);
        if data.is_anon {
            return FingerprintStyle::Opaque;
        }
        data.fingerprint_style
    }

    #[inline(always)]
    /// Return whether this kind always require evaluation.
    fn is_eval_always(self, kind: DepKind) -> bool {
        self.dep_kind_info(kind).is_eval_always
    }

    /// Try to force a dep node to execute and see if it's green.
    ///
    /// Returns true if the query has actually been forced. It is valid that a query
    /// fails to be forced, e.g. when the query key cannot be reconstructed from the
    /// dep-node or when the query kind outright does not support it.
    #[inline]
    #[instrument(skip(self, frame), level = "debug")]
    fn try_force_from_dep_node(self, dep_node: DepNode, frame: Option<&MarkFrame<'_>>) -> bool {
        let cb = self.dep_kind_info(dep_node.kind);
        if let Some(f) = cb.force_from_dep_node {
            match panic::catch_unwind(panic::AssertUnwindSafe(|| f(self, dep_node))) {
                Err(value) => {
                    if !value.is::<rustc_errors::FatalErrorMarker>() {
                        print_markframe_trace(self.dep_graph(), frame);
                    }
                    panic::resume_unwind(value)
                }
                Ok(query_has_been_forced) => query_has_been_forced,
            }
        } else {
            false
        }
    }

    /// Load data from the on-disk cache.
    fn try_load_from_on_disk_cache(self, dep_node: DepNode) {
        let cb = self.dep_kind_info(dep_node.kind);
        if let Some(f) = cb.try_load_from_on_disk_cache {
            f(self, dep_node)
        }
    }
}

pub trait Deps {
    /// Execute the operation with provided dependencies.
    fn with_deps<OP, R>(deps: TaskDepsRef<'_>, op: OP) -> R
    where
        OP: FnOnce() -> R;

    /// Access dependencies from current implicit context.
    fn read_deps<OP>(op: OP)
    where
        OP: for<'a> FnOnce(TaskDepsRef<'a>);

    /// We use this for most things when incr. comp. is turned off.
    const DEP_KIND_NULL: DepKind;

    /// We use this to create a forever-red node.
    const DEP_KIND_RED: DepKind;

    /// This is the highest value a `DepKind` can have. It's used during encoding to
    /// pack information into the unused bits.
    const DEP_KIND_MAX: u16;
}

pub trait HasDepContext: Copy {
    type Deps: self::Deps;
    type DepContext: self::DepContext<Deps = Self::Deps>;

    fn dep_context(&self) -> &Self::DepContext;
}

impl<T: DepContext> HasDepContext for T {
    type Deps = T::Deps;
    type DepContext = Self;

    fn dep_context(&self) -> &Self::DepContext {
        self
    }
}

impl<T: HasDepContext, Q: Copy> HasDepContext for (T, Q) {
    type Deps = T::Deps;
    type DepContext = T::DepContext;

    fn dep_context(&self) -> &Self::DepContext {
        self.0.dep_context()
    }
}

/// Describes the contents of the fingerprint generated by a given query.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FingerprintStyle {
    /// The fingerprint is actually a DefPathHash.
    DefPathHash,
    /// The fingerprint is actually a HirId.
    HirId,
    /// Query key was `()` or equivalent, so fingerprint is just zero.
    Unit,
    /// Some opaque hash.
    Opaque,
}

impl FingerprintStyle {
    #[inline]
    pub fn reconstructible(self) -> bool {
        match self {
            FingerprintStyle::DefPathHash | FingerprintStyle::Unit | FingerprintStyle::HirId => {
                true
            }
            FingerprintStyle::Opaque => false,
        }
    }
}