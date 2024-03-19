use dominator::{svg, Dom};

pub fn _render_svg_menu_icon(color: &str, size: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-menu"><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>
    svg!("svg", {
        .attr("alt", "menu icon")
        .attr("width", size)
        .attr("height", size)
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

pub fn _render_svg_help_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-help-circle"><circle cx="12" cy="12" r="10"></circle><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
    svg!("svg", {
        .attr("alt", "menu icon")
        .attr("width", size)
        .attr("height", size)
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
            svg!("path",{
                .attr("d", "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3")
            }),
            svg!("line", {
                .attr("x1", "12")
                .attr("y1", "17")
                .attr("x2", "12.01")
                .attr("y2", "17")
            }),
        ])
    })
}

pub fn render_svg_crosshair_icon(color: &str, size: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-crosshair"><circle cx="12" cy="12" r="10"></circle><line x1="22" y1="12" x2="18" y2="12"></line><line x1="6" y1="12" x2="2" y2="12"></line><line x1="12" y1="6" x2="12" y2="2"></line><line x1="12" y1="22" x2="12" y2="18"></line></svg>
    svg!("svg", {
        .attr("alt", "crosshair icon")
        .attr("width", size)
        .attr("height", size)
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

pub fn render_svg_external_link_icon(color: &str, size: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-external-link"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><polyline points="15 3 21 3 21 9"></polyline><line x1="10" y1="14" x2="21" y2="3"></line></svg>
    svg!("svg", {
        .attr("alt", "external link")
        .attr("width", size)
        .attr("height", size)
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

pub fn render_svg_arrow_up_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-arrow-up"><line x1="12" y1="19" x2="12" y2="5"></line><polyline points="5 12 12 5 19 12"></polyline></svg>
    svg!("svg", {
        .attr("alt", "up arrow")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("line", {
                .attr("x1", "12")
                .attr("y1", "19")
                .attr("x2", "12")
                .attr("y2", "5")
            }),
            svg!("polyline",{
                .attr("points", "5 12 12 5 19 12")
            }),
        ])
    })
}

pub fn render_svg_arrow_down_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-arrow-down"><line x1="12" y1="5" x2="12" y2="19"></line><polyline points="19 12 12 19 5 12"></polyline></svg>
    svg!("svg", {
        .attr("alt", "down arrow")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("line", {
                .attr("x1", "12")
                .attr("y1", "5")
                .attr("x2", "12")
                .attr("y2", "19")
            }),
            svg!("polyline",{
                .attr("points", "19 12 12 19 5 12")
            }),
        ])
    })
}

pub fn _render_svg_arrow_right_icon(color: &str, size: &str) -> Dom {
    //<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-arrow-right"><line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline></svg>
    svg!("svg", {
        .attr("alt", "right arrow")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("line", {
                .attr("x1", "5")
                .attr("y1", "12")
                .attr("x2", "19")
                .attr("y2", "12")
            }),
            svg!("polyline",{
                .attr("points", "12 5 19 12 12 19")
            }),
        ])
    })
}

pub fn render_svg_twitter_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-twitter"><path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"></path></svg>
    svg!("svg", {
        .attr("alt", "twitter icon")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", color)
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("path",{
                .attr("d", "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z")
            }),
        ])
    })
}

pub fn render_svg_facebook_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-facebook"><path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path></svg>
    svg!("svg", {
        .attr("alt", "facebook icon")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("path",{
                .attr("d", "M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z")
            }),
        ])
    })
}

pub fn render_svg_instagram_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-instagram"><rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect><path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path><line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line></svg>
    svg!("svg", {
        .attr("alt", "instagram icon")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("rect", {
                .attr("x", "2")
                .attr("y", "2")
                .attr("width", "20")
                .attr("height", "20")
                .attr("rx", "5")
                .attr("ry", "5")
            }),
            svg!("path",{
                .attr("d", "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z")
            }),
            svg!("line", {
                .attr("x1", "17.5")
                .attr("y1", "6.5")
                .attr("x2", "17.51")
                .attr("y2", "6.5")
            }),
        ])
    })
}

pub fn render_svg_linkedin_icon(color: &str, size: &str) -> Dom {
    // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-linkedin"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path><rect x="2" y="9" width="4" height="12"></rect><circle cx="4" cy="4" r="2"></circle></svg>
    svg!("svg", {
        .attr("alt", "linkedin icon")
        .attr("width", size)
        .attr("height", size)
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", color)
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .children(&mut[
            svg!("rect", {
                .attr("x", "2")
                .attr("y", "9")
                .attr("width", "4")
                .attr("height", "12")
            }),
            svg!("path",{
                .attr("d", "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z")
            }),
            svg!("circle", {
                .attr("cx", "4")
                .attr("cy", "4")
                .attr("r", "2")
            }),
        ])
    })
}
