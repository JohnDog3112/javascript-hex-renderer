type Color = [number, number, number, number];

namespace Lines {
    export interface Monocolor {
        type: "Monocolor",
        color: Color,
        bent: boolean
    }

    export interface Gradient {
        type: "Gradient",
        colors: Color[],
        segments_per_color: number,
        bent: boolean,
    }

    export interface SegmentColors {
        type: "SegmentColors",
        colors: Color[],
        triangles: Triangle.Any,
        collisions: CollisionOption.Any
    }

    export type Any = Monocolor | Gradient | SegmentColors;
}



namespace Triangle {
    export interface None {
        type: "None"
    }

    export interface Match {
        type: "Match",
        radius: number
    }

    export interface BorderMatch {
        type: "BorderMatch",
        match_radius: number,
        border: Marker
    }

    export interface BorderStartMatch {
        type: "BorderStartMatch",
        match_radius: number,
        border: Marker
    }

    export type Any = None | Match | BorderMatch | BorderStartMatch;
}

namespace CollisionOption {
    export interface Dashes {
        type: "Dashes",
        color: Color,
    }

    export interface MatchedDashes {
        type: "MatchedDashes"
    }

    export interface ParallelLines {
        type: "ParallelLines"
    }

    export interface OverloadedParallel {
        type: "OverloadedParallel",
        max_line: number,
        overload: OverloadOptions.Any
    }

    export type Any = Dashes | MatchedDashes | ParallelLines | OverloadedParallel;
}


namespace OverloadOptions {
    export interface Dashes {
        type: "Dashes",
        color: Color,
    }

    export interface LabeledDashes {
        type: "LabeledDashes",
        color: Color,
        label: Marker
    }

    export interface MatchedDashes {
        type: "MatchedDashes"
    }

    export type Any = Dashes | LabeledDashes | MatchedDashes;
}

namespace Point {
    export interface None {
        type: "None"
    }

    export interface Single {
        type: "Single",
        marker: Marker,
    }

    export interface Double {
        type: "Double",
        inner: Marker,
        outer: Marker,
    }
    
    export type Any = None | Single | Double;
}

interface Marker {
    color: Color,
    radius: number
}

namespace EndPoint {
    export interface Point {
        type: "Point",
        point: Point,
    }

    export interface Match {
        type: "Match",
        radius: number,
    }

    export interface BorderedMatch {
        type: "BorderedMatch",
        match_radius: number,
        border: number
    }

    export type Any = Point | Match | BorderedMatch;
}

namespace Intersections {
    export interface Nothing {
        type: "Nothing"
    }

    export interface UniformPoints {
        type: "UniformPoints",
        point: Point.Any,
    }

    export interface EndsAndMiddle {
        start: EndPoint.Any,
        end: EndPoint.Any,
        middle: Point.Any,
    }

    export type Any = Nothing | UniformPoints | EndsAndMiddle;
}

namespace GridPatternOptions {
    export interface Uniform {
        type: "Uniform",
        intersections: Intersections.Any,
        lines: Lines.Any,
    }

    export interface Changing {
        type: "Changing",
        variations: [Intersections.Any, Lines.Any][],
        intros: String[],
        retros: String[],
    }

    export type Any = Uniform | Changing;
}

interface GridOptions {
    line_thickness: number,
    pattern_options: GridPatternOptions.Any,
    center_dot: Point.Any,
}


type Direction = "East" | "NorthEast" | "NorthWest" | "West" | "SouthWest" | "SouthEast";

interface PatternVariant {
    direction: Direction,
    angle_sigs: String,
    great_spell: boolean,
}


const inter: Intersections.Any = {
    type: "UniformPoints",
    point: {
        type: "Single",
        marker: {
            color: [255, 255, 255, 255],
            radius: 0.07,
        }
    }
};

const grid: GridOptions = {
    line_thickness: 0.12,
    pattern_options: {
        type: "Changing",
        variations: [
            [
                {
                    type: "UniformPoints",
                    point: {
                        type: "Single",
                        marker: {
                            color: [255, 255, 255, 255],
                            radius: 0.07,
                        }
                    }
                },
                {
                    type: "Monocolor",
                    color: [255, 0, 0, 255],
                    bent: true
                }
            ],
            [
                {
                    type: "UniformPoints",
                    point: {
                        type: "Single",
                        marker: {
                            color: [255, 255, 255, 255],
                            radius: 0.07,
                        }
                    }
                },
                {
                    type: "Monocolor",
                    color: [0, 255, 0, 255],
                    bent: true
                }
            ]
        ],
        intros: ["qqq"],
        retros: ["eee"],
    },

    center_dot: {
        type: "None"
    }
}


const intro_pattern: PatternVariant = {
    direction: "East",
    angle_sigs: "qqq",
    great_spell: false
};

const retro_pattern: PatternVariant = {
    direction: "East",
    angle_sigs: "eee",
    great_spell: false
    
};

const patterns: PatternVariant[] = [
    {
        direction: "West",
        angle_sigs: "qqq",
        great_spell: false
    },
    {
        direction: "West",
        angle_sigs: "qqq",
        great_spell: false
    },
    {
        direction: "West",
        angle_sigs: "qqq",
        great_spell: false
    },
    {
        direction: "West",
        angle_sigs: "qqq",
        great_spell: false
    },
    {
        direction: "West",
        angle_sigs: "qqq",
        great_spell: false
    },

    {
        direction: "East",
        angle_sigs: "",
        great_spell: false
    },

    {
        direction: "East",
        angle_sigs: "eee",
        great_spell: false
    },
    {
        direction: "East",
        angle_sigs: "eee",
        great_spell: false
    },
    {
        direction: "East",
        angle_sigs: "eee",
        great_spell: false
    },
    {
        direction: "East",
        angle_sigs: "eee",
        great_spell: false
    },
    {
        direction: "East",
        angle_sigs: "eee",
        great_spell: false
    }
]
