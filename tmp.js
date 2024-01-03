const bindings = await import("/bindings.js");

let colors = [new bindings.Color(255, 0, 0, 255), new bindings.Color(0, 255, 0, 255), new bindings.Color(0, 0, 255, 255)];

let lines = new bindings.Lines.Gradient(colors, 10, true);

let point_marker = new bindings.Marker(new bindings.Color(255, 255, 255, 255), 0.07); 

let point = new bindings.Point.Single(point_marker);

let intersections = new bindings.Intersections.UniformPoints(point);

let grid_pattern_options = new bindings.GridPatternOptions.Uniform(intersections, lines); 

let grid_options = new bindings.GridOptions(0.12, grid_pattern_options, new bindings.Point.None);

let pattern = new bindings.PatternVariant.Normal("NORTH_EAST", "qeqwqwqwqwqeqssssaeqeaqeqaeqaqdededwaqdedsssssdssess"); 

let img_data = bindings.default.draw_hex_grid(grid_options, [pattern], 50, 50); 

let img = new Image();
img.src = URL.createObjectURL(new Blob([img_data.buffer], { type: 'image/png' }));
document.body.appendChild(img);