#[derive(Clone)]
pub struct MobileRoutes {
    pub home: &'static str,
    pub index: &'static str,
    pub prefix: &'static str,
    pub scores: &'static str,
    pub vote: &'static str,
    pub voters: &'static str,
}

#[derive(Clone)]
pub struct WebRoutes {
    pub index: &'static str,
    pub results: &'static str,
    pub vote: &'static str,
    pub json: &'static str,
    pub v1: &'static str,
    pub mobile: MobileRoutes,
}
pub const WEB_ROUTES: WebRoutes = WebRoutes {
    index: "/",
    results: "/results",
    vote: "/vote",
    json: "/json",
    v1: "/v1",
    mobile: MobileRoutes {
        prefix: "/hyperview/public",
        index: "index.xml",
        home: "home.xml",
        scores: "scores.xml",
        vote: "vote", // POST
        voters: "voters.xml",
    },
};
