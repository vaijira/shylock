use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "#properties"]
    Properties,
    #[to = "#home"]
    HomePage,
}
/// A wrapper around a Route that enables fragment-only routing
///
/// When analyzing the current address, the route considers only the fragment identifier.
/// Conversely, all routes must start with a '#' sign rather than a slash.
///
/// This is useful for applications that are expected to be shipped as static files to any file
/// server, are supposed to work from any file name, and do not require any configuration on the
/// server side.
#[derive(Clone)]
pub struct FragmentOnlyRoute<I: Switch> {
    pub inner: I,
}

impl<I: Switch> Switch for FragmentOnlyRoute<I> {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        let part = match part.find('#') {
            Some(i) => &part[i..],
            None => "",
        }.to_string();
        let (slef, outstate) = I::from_route_part(part, state);
        (slef.map(|s| s.into()), outstate)
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        // No further adjustments are needed: As the inner route produces URI refrences starting
        // with a '#', they can just be applied and do not change the resource.
        self.inner.build_route_section(route)
    }
}

impl<I: Switch> From<I> for FragmentOnlyRoute<I> {
    fn from(inner: I) -> Self {
        Self { inner }
    }
}

