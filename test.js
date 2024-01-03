let grid_options = {
    line_thickness: 0.12,
    grid_pattern_options: {
        intersections: {
            type: "UniformPoints",
            point: {
                "type": "Single",
                "marker": {
                    color: [255, 255, 255, 255],
                    radius: 0.07
                }
            }
        },
        lines: {
            type: "Gradient",
            colors: [
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0, 0, 255, 255]
            ],
            bent: true
        }
    },
    center_point: {
        "type": "None"
    }
}