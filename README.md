# Hex Renderer
A bridge librarary via wasm for [Hex Renderer](https://github.com/JohnDog3112/Hex-Renderer) and javascript
The NPM package is available [here](https://www.npmjs.com/package/hex_renderer_javascript)

If you want to use this on the web, the url for version 0.1.2 is [https://cdn.jsdelivr.net/npm/hex_renderer_javascript@0.1.2/hex_renderer_javascript.js](https://cdn.jsdelivr.net/npm/hex_renderer_javascript@0.1.2/hex_renderer_javascript.js)

# Main Functions

All of the given functions take in a set of drawing options and patterns to render them as a PNG and they output it as an array of bytes.

## Hex Grid
```ts
function draw_hex_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: number, scale: number): Uint8Array;
```
The Hex Grid draws all of the patterns on a hexagonal grid. The max_width is the maximum width of the hexagonal grid (before patterns wrap around), and the scale is the distance between grid points in pixels.

## Square Grid
```ts
function draw_square_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: number, max_scale: number, x_pad: number, y_pad: number, scale: number): Uint8Array;
```
The Square Grid arranges all of the patterns as a grid of tiles. The max_width is how many tiles are in each row. Each pattern dynamically scales to fit in a tile, but max_width sets an upper bound (as a percentage) for how large they can scale up. The x_pad and y_pad parameters control the x and y padding respectively between tiles and are a percentage of the scale. Lastly, scale is the size (in pixels) of the tiles.

## Bound versions
```ts
function draw_bound_hex_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: number, width: number, height: number): Uint8Array;

function draw_bound_square_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: number, max_scale: number, x_pad: number, y_pad: number, width: number, height: number): Uint8Array;

```
The bound versions of Hex grid and Square grid are essentially the same as above. The only difference is that scale is replaced by width and height which specify the bounds the resulting image must fit in.

## Single Pattern
```ts
function draw_bound_pattern(grid_options: GridOptions, pattern: PatternVariant, max_scale: number, width: number, height: number): Uint8Array;
```
Very similar to draw_bound_square_grid, except it only draws a single pattern. In addition, it does not add padding for the arrows/triangles in the segment renderer. This allows for the size of the different renderers be the same as long as everything else stays constant.

# Patterns
```ts
interface PatternVariant {
    direction: string;
    angle_sigs: string;
    great_spell: boolean;
}
```
A pattern is made up of 3 attributes, the direction (East, NorthEast, etc.), the angle_sigs ("qqqq", "qawdqw", etc.) and if it's a great spell. If it's a great spell, it will be drawn monocolor (using the first color in the given color(s)) and won't show any signs of direction.

# GridOptions
This section still needs to be worked on, but type checking via JSDOCS and typescript should work. Just keep in mind that several of the subtypes require the field type to be included before it gives full type hints.

## Examples:
```ts
const monocolor = {
    line_thickness: 0.12,
    center_dot: {
        type: "None"
    },
    pattern_options: {
        type: "Uniform",
        lines: {
            type: "Monocolor",
            color: [214, 9, 177, 255],
            bent: true
        },
        intersections: {
            type: "UniformPoints",
            point: {
                type: "Single",
                marker: {
                    color: [255, 255, 255, 255],
                    radius: 0.07,
                }
            }
        }
    }
}
```

```ts
const gradient = {
    line_thickness: 0.12,
    center_dot: {
        type: "None"
    },
    pattern_options: {
        type: "Uniform",
        lines: {
            type: "Gradient",
            colors: [
                [214, 9, 177, 255],
                [108, 25, 140, 255],
                [50, 102, 207, 255],
                [102, 110, 125, 255],
            ],
            bent: true,
            segments_per_color: 15,
        },
        intersections: {
            type: "UniformPoints",
            point: {
                type: "Single",
                marker: {
                    color: [255, 255, 255, 255],
                    radius: 0.07,
                }
            }
        }
    },
}
```

```ts
const segment = {
    line_thickness: 0.12,
    center_dot: {
        type: "None"
    },
    pattern_options: {
        type: "Uniform",
        intersections: {
            type: "UniformPoints",
            point: {
                type: "Single",
                marker: {
                    color: [255, 255, 255, 255],
                    radius: 0.07,
                }
            }
        },
        lines: {
            type: "SegmentColors",
            colors: [
                [214, 9, 177, 255],
                [108, 25, 140, 255],
                [50, 102, 207, 255],
                [102, 110, 125, 255],
            ],
            triangles: {
                type: "BorderStartMatch",
                match_radius: 0.16,
                border: {
                    color: [255, 255, 255, 255],
                    radius: 0.25,
                }
            },
            collisions: {
                type: "Dashes",
                color: [255, 0, 0, 255],
            }
        }
    }
}
```