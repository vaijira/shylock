use dominator::{svg, Dom};

pub fn _render_svg_menu_icon(color: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-menu"><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>
    svg!("svg", {
        .attr("width", "24")
        .attr("height", "24")
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("line", {
                .attr("x1", "3")
                .attr("y1", "6")
                .attr("x2", "21")
                .attr("y2", "6")
            }),
            svg!("line",{
                .attr("x1", "3")
                .attr("y1", "12")
                .attr("x2", "21")
                .attr("y2", "12")
            }),
            svg!("line", {
                .attr("x1", "3")
                .attr("y1", "18")
                .attr("x2", "21")
                .attr("y2", "18")
            }),
        ])
    })
}

pub fn render_svg_crosshair_icon(color: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-crosshair"><circle cx="12" cy="12" r="10"></circle><line x1="22" y1="12" x2="18" y2="12"></line><line x1="6" y1="12" x2="2" y2="12"></line><line x1="12" y1="6" x2="12" y2="2"></line><line x1="12" y1="22" x2="12" y2="18"></line></svg>
    svg!("svg", {
        .attr("width", "24")
        .attr("height", "24")
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("circle", {
                .attr("cx", "12")
                .attr("cy", "12")
                .attr("r", "10")
            }),
            svg!("line", {
                .attr("x1", "22")
                .attr("y1", "12")
                .attr("x2", "18")
                .attr("y2", "12")
            }),
            svg!("line",{
                .attr("x1", "6")
                .attr("y1", "12")
                .attr("x2", "2")
                .attr("y2", "12")
            }),
            svg!("line", {
                .attr("x1", "12")
                .attr("y1", "6")
                .attr("x2", "12")
                .attr("y2", "2")
            }),
            svg!("line", {
                .attr("x1", "12")
                .attr("y1", "22")
                .attr("x2", "12")
                .attr("y2", "18")
            }),
        ])
    })
}

pub fn render_svg_external_link_icon(color: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-external-link"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><polyline points="15 3 21 3 21 9"></polyline><line x1="10" y1="14" x2="21" y2="3"></line></svg>
    svg!("svg", {
        .attr("width", "24")
        .attr("height", "24")
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("path", {
                .attr("d", "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6")
            }),
            svg!("polyline", {
                .attr("points", "15 3 21 3 21 9")
            }),
            svg!("line",{
                .attr("x1", "10")
                .attr("y1", "14")
                .attr("x2", "21")
                .attr("y2", "3")
            }),
        ])
    })
}
