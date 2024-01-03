let grid_options = {
    line_thickness: 0.12,
    pattern_options: {
        type: "Uniform",
        intersections: {
            type: "UniformPoints",
            point: {
                type: "Single",
                marker: {
                    color: [255, 255, 255, 255],
                    radius: 0.07
                }
            }
        },
        lines: {
            type: "Gradient",
            segments_per_color: 5,
            colors: [
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0, 0, 255, 255]
            ],
            bent: true
        }
    },
    center_dot: {
        type: "None"
    }
}