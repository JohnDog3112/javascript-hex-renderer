class Color {
    #r;
    #g;
    #b;
    #a;
    constructor(r, g, b, a) {
        this.#r = r;
        this.#g = g;
        this.#b = b;
        this.#a = a;
    }

    get get() {
        return [this.#r, this.#g, this.#b, this.#a]
    }
}

class Marker {
    constructor(color, radius) {
        this.#color = color;
        this.#radius = radius;
    }

    #color;
	get color() {
        return this.#color;
    }

    #radius;
	get radius() {
        return this.#radius;
    }
}

let Point = {};

Point.None = class {
    get type() {
        return 0;
    }
}

Point.Single = class {
    constructor(marker) {
        this.#marker = marker;
    }

    get type() {
        return 1;
    }

    #marker;
	get first() {
        return this.#marker;
    }
}

Point.Double = class {
    constructor(first_marker, second_marker) {
        this.#first = first_marker;
        this.#second = second_marker;
    }
    get type() {
        return 2;
    }

    #first;
	get first() {
        return this.#first;
    }

    #second;
	get second() {
        return this.#second;
    }
}

let Triangle = {};

Triangle.None = class {
    get type() {
        return 0;
    }
}

Triangle.Match = class {
    constructor(radius) {
        this.#radius = radius;
    }

    get type() {
        return 1;
    }

    #radius;
	get radius() {
        return this.#radius;
    }
}

Triangle.BorderMatch = class {
    constructor(radius, border_marker) {
        this.#radius = radius;
        this.#border = border_marker;
    } 

    get type() {
        return 2;
    }

    #radius;
	get radius() {
        return this.#radius;
    }

    #border;
	get border() {
        return this.#border;
    }
}

Triangle.BorderStartMatch = class {
    constructor(radius, border_marker) {
        this.#radius = radius;
        this.#border = border_marker;
    } 

    get type() {
        return 3;
    }

    #radius;
	get radius() {
        return this.#radius;
    }

    #border;
	get border() {
        return this.#border;
    }
}

let OverloadOptions = {};

OverloadOptions.Dashes = class {
    constructor(color) {
        this.#color = color;
    }

    get type() {
        return 0;
    }

    #color;
	get color() {
        return this.#color;
    }
}

OverloadOptions.LabeledDashes = class {
    constructor(color, label) {
        this.#color = color;
        this.#label = label;
    }

    get type() {
        return 1;
    }

    #color;
	get color() {
        return this.#color;
    }

    #label;
	get label() {
        return this.#label;
    }
}

OverloadOptions.MatchedDashes = class {
    get type() {
        return 2;
    }
}


let CollisionOption = {};

CollisionOption.Dashes = class {
    constructor(color) {
        this.#color = color;
    }

    get type() {
        return 0;
    }

    #color;
	get color() {
        return this.#color;
    }
}

CollisionOption.MatchedDashes = class {
    get type() {
        return 1;
    }
}

CollisionOption.ParallelLines = class {
    get type() {
        return 2;
    }
}

CollisionOption.OverloadedParallel = class {
    constructor(max_line, overload) {
        this.#max_line = max_line;
        this.#overload = overload;
    }

    get type() {
        return 3;
    }

    #max_line;
	get max_line() {
        return this.#max_line;
    }

    #overload;
	get overload() {
        return this.#overload;
    }
}


let Lines = {};

Lines.Monocolor = class {
    constructor(color, bent) {
        this.#color = color;
        this.#bent = bent;
    }

    get type() {
        return 0;
    }

    #color;
	get color() {
        return this.#color;
    }

    #bent;
	get bent() {
        return this.#bent;
    }
}

Lines.Gradient = class {
    constructor(colors, segments_per_color, bent) {
        this.#colors = colors;
        this.#segments_per_color = segments_per_color;
        this.#bent = bent;
    }

    get type() {
        return 1;
    }

    #colors;
	get colors() {
        return this.#colors;
    }

    #segments_per_color;
	get segments_per_color() {
        return this.#segments_per_color;
    }

    #bent;
	get bent() {
        return this.#bent;
    }
}

Lines.SegmentColors = class {
    constructor(colors, triangles, collisions) {
        this.#colors = colors;
        this.#triangles = triangles;
        this.#collisions = collisions;
    }

    get type() {
        return 2;
    }

    #triangles;
	get triangles() {
        return this.#triangles;
    }

    #collisions;
	get collisions() {
        return this.#collisions;
    }

    #colors;
    get colors() {
        return this.#colors;
    }
}


let EndPoint = {};

EndPoint.Point = class {
    constructor(point) {
        this.#point = point;
    }

    get type() {
        return 0;
    }

    #point;
	get point() {
        return this.#point;
    }
}

EndPoint.Match = class {
    constructor(radius) {
        this.#radius = radius;
    }

    get type() {
        return 1;
    }

    #radius;
	get radius() {
        return this.#radius;
    }
}

EndPoint.BorderMatch = class {
    constructor(radius, border) {
        this.#radius = radius;
        this.#border = border;
    }

    get type() {
        return 2;
    }

    #radius;
	get radius() {
        return this.#radius;
    }

    #border;
	get border() {
        return this.#border;
    }
}


let Intersections = {};

Intersections.Nothing = class {
    get type() {
        return 0;
    }
};

Intersections.UniformPoints = class {
    constructor(point) {
        this.#point = point;
    }

    get type() {
        return 1;
    }

    #point;
	get point() {
        return this.#point;
    }
};

Intersections.EndsAndMiddle = class {
    constructor(start, middle, end) {
        this.#start = start;
        this.#middle = middle;
        this.#end = end;
    }

    get type() {
        return 2;
    }

    #start;
	get start() {
        return this.#start;
    }

    #middle;
	get middle() {
        return this.#middle;
    }

    #end;
	get end() {
        return this.#end;
    }
}


let GridPatternOptions = {};

GridPatternOptions.Uniform = class {
    constructor(intersections, lines) {
        this.#intersections = intersections;
        this.#lines = lines;
    }

    get type() {
        return 0;
    }

    #intersections;
	get intersections() {
        return this.#intersections;
    }

    #lines;
	get lines() {
        return this.#lines;
    }
}

GridPatternOptions.Changing = class {
    constructor(intersections, lines, intros, retros) {
        this.#intersections = intersections;
        this.#lines = lines;
        this.#intros = intros;
        this.#retros = retros;
    }

    get type() {
        return 1;
    }

    #intersections;
	get intersections() {
        return this.#intersections;
    }

    #lines;
	get lines() {
        return this.#lines;
    }

    #intros;
	get intros() {
        return this.#intros;
    }

    #retros;
	get retros() {
        return this.#retros;
    }
}


class GridOptions {
    constructor(line_thickness, pattern_options, center_dot) {
        this.#line_thickness = line_thickness;
        this.#pattern_options = pattern_options;
        this.#center_dot = center_dot;
    }

    #line_thickness;
	get line_thickness() {
        return this.#line_thickness;
    }

    #pattern_options;
	get pattern_options() {
        return this.#pattern_options;
    }

    #center_dot;
	get center_dot() {
        return this.#center_dot;
    }
}

class Pattern {
    constructor(direction, angles) {
        this.#direction = direction;
        this.#angles = angles;
    }

    #direction;
	get direction() {
        return this.#direction;
    }

    #angles;
	get angles() {
        return this.#angles;
    }
}

let PatternVariant = {};

PatternVariant.Normal = class extends Pattern {
    constructor(direction, angles) {
        super(direction, angles);
    }

    get type() {
        return 0;
    }
}

PatternVariant.Monocolor = class extends Pattern {
    constructor(direction, angles) {
        super(direction, angles);
    }

    get type() {
        return 1;
    }
}

function init_renderer() {
    let a = import("/pkg/hex_renderer_javascript.js");

    return (async () => {
        let renderer = await a;
        await renderer.default()
        return renderer;
    })();
}

let renderer = init_renderer();

export { Color, Marker, Point, Triangle, OverloadOptions, CollisionOption, Lines, EndPoint, Intersections, GridPatternOptions, GridOptions, PatternVariant};

export default await renderer;