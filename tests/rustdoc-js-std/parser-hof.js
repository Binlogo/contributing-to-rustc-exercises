const PARSED = [
    // ML-style HOF
    {
        query: "(-> F<P>)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [],
            bindings: [
                [
                    "output",
                    [{
                        name: "f",
                        fullPath: ["f"],
                        pathWithoutLast: [],
                        pathLast: "f",
                        generics: [
                            {
                                name: "p",
                                fullPath: ["p"],
                                pathWithoutLast: [],
                                pathLast: "p",
                                generics: [],
                            },
                        ],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(-> F<P>)",
        returned: [],
        userQuery: "(-> f<p>)",
        error: null,
    },
    {
        query: "(-> P)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [],
            bindings: [
                [
                    "output",
                    [{
                        name: "p",
                        fullPath: ["p"],
                        pathWithoutLast: [],
                        pathLast: "p",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(-> P)",
        returned: [],
        userQuery: "(-> p)",
        error: null,
    },
    {
        query: "(->,a)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [],
            bindings: [
                [
                    "output",
                    [{
                        name: "a",
                        fullPath: ["a"],
                        pathWithoutLast: [],
                        pathLast: "a",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(->,a)",
        returned: [],
        userQuery: "(->,a)",
        error: null,
    },
    {
        query: "(F<P> ->)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [{
                name: "f",
                fullPath: ["f"],
                pathWithoutLast: [],
                pathLast: "f",
                generics: [
                    {
                        name: "p",
                        fullPath: ["p"],
                        pathWithoutLast: [],
                        pathLast: "p",
                        generics: [],
                    },
                ],
                typeFilter: -1,
            }],
            bindings: [
                [
                    "output",
                    [],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(F<P> ->)",
        returned: [],
        userQuery: "(f<p> ->)",
        error: null,
    },
    {
        query: "(P ->)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [{
                name: "p",
                fullPath: ["p"],
                pathWithoutLast: [],
                pathLast: "p",
                generics: [],
                typeFilter: -1,
            }],
            bindings: [
                [
                    "output",
                    [],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(P ->)",
        returned: [],
        userQuery: "(p ->)",
        error: null,
    },
    {
        query: "(,a->)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [{
                name: "a",
                fullPath: ["a"],
                pathWithoutLast: [],
                pathLast: "a",
                generics: [],
                typeFilter: -1,
            }],
            bindings: [
                [
                    "output",
                    [],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(,a->)",
        returned: [],
        userQuery: "(,a->)",
        error: null,
    },
    {
        query: "(aaaaa->a)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [{
                name: "aaaaa",
                fullPath: ["aaaaa"],
                pathWithoutLast: [],
                pathLast: "aaaaa",
                generics: [],
                typeFilter: -1,
            }],
            bindings: [
                [
                    "output",
                    [{
                        name: "a",
                        fullPath: ["a"],
                        pathWithoutLast: [],
                        pathLast: "a",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(aaaaa->a)",
        returned: [],
        userQuery: "(aaaaa->a)",
        error: null,
    },
    {
        query: "(aaaaa, b -> a)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [
                {
                    name: "aaaaa",
                    fullPath: ["aaaaa"],
                    pathWithoutLast: [],
                    pathLast: "aaaaa",
                    generics: [],
                    typeFilter: -1,
                },
                {
                    name: "b",
                    fullPath: ["b"],
                    pathWithoutLast: [],
                    pathLast: "b",
                    generics: [],
                    typeFilter: -1,
                },
            ],
            bindings: [
                [
                    "output",
                    [{
                        name: "a",
                        fullPath: ["a"],
                        pathWithoutLast: [],
                        pathLast: "a",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(aaaaa, b -> a)",
        returned: [],
        userQuery: "(aaaaa, b -> a)",
        error: null,
    },
    {
        query: "primitive:(aaaaa, b -> a)",
        elems: [{
            name: "->",
            fullPath: ["->"],
            pathWithoutLast: [],
            pathLast: "->",
            generics: [
                {
                    name: "aaaaa",
                    fullPath: ["aaaaa"],
                    pathWithoutLast: [],
                    pathLast: "aaaaa",
                    generics: [],
                    typeFilter: -1,
                },
                {
                    name: "b",
                    fullPath: ["b"],
                    pathWithoutLast: [],
                    pathLast: "b",
                    generics: [],
                    typeFilter: -1,
                },
            ],
            bindings: [
                [
                    "output",
                    [{
                        name: "a",
                        fullPath: ["a"],
                        pathWithoutLast: [],
                        pathLast: "a",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: 1,
        }],
        foundElems: 1,
        original: "primitive:(aaaaa, b -> a)",
        returned: [],
        userQuery: "primitive:(aaaaa, b -> a)",
        error: null,
    },
    {
        query: "x, trait:(aaaaa, b -> a)",
        elems: [
            {
                name: "x",
                fullPath: ["x"],
                pathWithoutLast: [],
                pathLast: "x",
                generics: [],
                typeFilter: -1,
            },
            {
                name: "->",
                fullPath: ["->"],
                pathWithoutLast: [],
                pathLast: "->",
                generics: [
                    {
                        name: "aaaaa",
                        fullPath: ["aaaaa"],
                        pathWithoutLast: [],
                        pathLast: "aaaaa",
                        generics: [],
                        typeFilter: -1,
                    },
                    {
                        name: "b",
                        fullPath: ["b"],
                        pathWithoutLast: [],
                        pathLast: "b",
                        generics: [],
                        typeFilter: -1,
                    },
                ],
                bindings: [
                    [
                        "output",
                        [{
                            name: "a",
                            fullPath: ["a"],
                            pathWithoutLast: [],
                            pathLast: "a",
                            generics: [],
                            typeFilter: -1,
                        }],
                    ],
                ],
                typeFilter: 10,
            }
        ],
        foundElems: 2,
        original: "x, trait:(aaaaa, b -> a)",
        returned: [],
        userQuery: "x, trait:(aaaaa, b -> a)",
        error: null,
    },
    // Rust-style HOF
    {
        query: "Fn () -> F<P>",
        elems: [{
            name: "fn",
            fullPath: ["fn"],
            pathWithoutLast: [],
            pathLast: "fn",
            generics: [],
            bindings: [
                [
                    "output",
                    [{
                        name: "f",
                        fullPath: ["f"],
                        pathWithoutLast: [],
                        pathLast: "f",
                        generics: [
                            {
                                name: "p",
                                fullPath: ["p"],
                                pathWithoutLast: [],
                                pathLast: "p",
                                generics: [],
                            },
                        ],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "Fn () -> F<P>",
        returned: [],
        userQuery: "fn () -> f<p>",
        error: null,
    },
    {
        query: "FnMut() -> P",
        elems: [{
            name: "fnmut",
            fullPath: ["fnmut"],
            pathWithoutLast: [],
            pathLast: "fnmut",
            generics: [],
            bindings: [
                [
                    "output",
                    [{
                        name: "p",
                        fullPath: ["p"],
                        pathWithoutLast: [],
                        pathLast: "p",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "FnMut() -> P",
        returned: [],
        userQuery: "fnmut() -> p",
        error: null,
    },
    {
        query: "(FnMut() -> P)",
        elems: [{
            name: "fnmut",
            fullPath: ["fnmut"],
            pathWithoutLast: [],
            pathLast: "fnmut",
            generics: [],
            bindings: [
                [
                    "output",
                    [{
                        name: "p",
                        fullPath: ["p"],
                        pathWithoutLast: [],
                        pathLast: "p",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "(FnMut() -> P)",
        returned: [],
        userQuery: "(fnmut() -> p)",
        error: null,
    },
    {
        query: "Fn(F<P>)",
        elems: [{
            name: "fn",
            fullPath: ["fn"],
            pathWithoutLast: [],
            pathLast: "fn",
            generics: [{
                name: "f",
                fullPath: ["f"],
                pathWithoutLast: [],
                pathLast: "f",
                generics: [
                    {
                        name: "p",
                        fullPath: ["p"],
                        pathWithoutLast: [],
                        pathLast: "p",
                        generics: [],
                    },
                ],
                typeFilter: -1,
            }],
            bindings: [
                [
                    "output",
                    [],
                ],
            ],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "Fn(F<P>)",
        returned: [],
        userQuery: "fn(f<p>)",
        error: null,
    },
    {
        query: "primitive:fnonce(aaaaa, b) -> a",
        elems: [{
            name: "fnonce",
            fullPath: ["fnonce"],
            pathWithoutLast: [],
            pathLast: "fnonce",
            generics: [
                {
                    name: "aaaaa",
                    fullPath: ["aaaaa"],
                    pathWithoutLast: [],
                    pathLast: "aaaaa",
                    generics: [],
                    typeFilter: -1,
                },
                {
                    name: "b",
                    fullPath: ["b"],
                    pathWithoutLast: [],
                    pathLast: "b",
                    generics: [],
                    typeFilter: -1,
                },
            ],
            bindings: [
                [
                    "output",
                    [{
                        name: "a",
                        fullPath: ["a"],
                        pathWithoutLast: [],
                        pathLast: "a",
                        generics: [],
                        typeFilter: -1,
                    }],
                ],
            ],
            typeFilter: 1,
        }],
        foundElems: 1,
        original: "primitive:fnonce(aaaaa, b) -> a",
        returned: [],
        userQuery: "primitive:fnonce(aaaaa, b) -> a",
        error: null,
    },
    {
        query: "primitive:fnonce(aaaaa, keyword:b) -> trait:a",
        elems: [{
            name: "fnonce",
            fullPath: ["fnonce"],
            pathWithoutLast: [],
            pathLast: "fnonce",
            generics: [
                {
                    name: "aaaaa",
                    fullPath: ["aaaaa"],
                    pathWithoutLast: [],
                    pathLast: "aaaaa",
                    generics: [],
                    typeFilter: -1,
                },
                {
                    name: "b",
                    fullPath: ["b"],
                    pathWithoutLast: [],
                    pathLast: "b",
                    generics: [],
                    typeFilter: 0,
                },
            ],
            bindings: [
                [
                    "output",
                    [{
                        name: "a",
                        fullPath: ["a"],
                        pathWithoutLast: [],
                        pathLast: "a",
                        generics: [],
                        typeFilter: 10,
                    }],
                ],
            ],
            typeFilter: 1,
        }],
        foundElems: 1,
        original: "primitive:fnonce(aaaaa, keyword:b) -> trait:a",
        returned: [],
        userQuery: "primitive:fnonce(aaaaa, keyword:b) -> trait:a",
        error: null,
    },
    {
        query: "x, trait:fn(aaaaa, b -> a)",
        elems: [
            {
                name: "x",
                fullPath: ["x"],
                pathWithoutLast: [],
                pathLast: "x",
                generics: [],
                typeFilter: -1,
            },
            {
                name: "fn",
                fullPath: ["fn"],
                pathWithoutLast: [],
                pathLast: "fn",
                generics: [
                    {
                        name: "->",
                        fullPath: ["->"],
                        pathWithoutLast: [],
                        pathLast: "->",
                        generics: [
                            {
                                name: "aaaaa",
                                fullPath: ["aaaaa"],
                                pathWithoutLast: [],
                                pathLast: "aaaaa",
                                generics: [],
                                typeFilter: -1,
                            },
                            {
                                name: "b",
                                fullPath: ["b"],
                                pathWithoutLast: [],
                                pathLast: "b",
                                generics: [],
                                typeFilter: -1,
                            },
                        ],
                        bindings: [
                            [
                                "output",
                                [{
                                    name: "a",
                                    fullPath: ["a"],
                                    pathWithoutLast: [],
                                    pathLast: "a",
                                    generics: [],
                                    typeFilter: -1,
                                }],
                            ],
                        ],
                        typeFilter: -1,
                    },
                ],
                bindings: [
                    [
                        "output",
                        [],
                    ]
                ],
                typeFilter: 10,
            }
        ],
        foundElems: 2,
        original: "x, trait:fn(aaaaa, b -> a)",
        returned: [],
        userQuery: "x, trait:fn(aaaaa, b -> a)",
        error: null,
    },
    {
        query: 'a,b(c)',
        elems: [
            {
                name: "a",
                fullPath: ["a"],
                pathWithoutLast: [],
                pathLast: "a",
                generics: [],
                typeFilter: -1,
            },
            {
                name: "b",
                fullPath: ["b"],
                pathWithoutLast: [],
                pathLast: "b",
                generics: [{
                    name: "c",
                    fullPath: ["c"],
                    pathWithoutLast: [],
                    pathLast: "c",
                    generics: [],
                    typeFilter: -1,
                }],
                bindings: [
                    [
                        "output",
                        [],
                    ]
                ],
                typeFilter: -1,
            }
        ],
        foundElems: 2,
        original: "a,b(c)",
        returned: [],
        userQuery: "a,b(c)",
        error: null,
    },
];