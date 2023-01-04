//! This module defines the protocol of the trip service
//! 
//! The trip plugin solves the Traveling Salesman Problem using a greedy heuristic 
//! (farthest-insertion algorithm) for 10 or more waypoints and uses brute force for 
//! less than 10 waypoints. The returned path does not have to be the fastest path. 
//! As TSP is NP-hard it only returns an approximation. Note that all input coordinates 
//! have to be connected for the trip service to work.
//! 
//! It is possible to explicitely set the start or end coordinate of the trip. 
//! When source is set to first, the first coordinate is used as start coordinate of the 
//! trip in the output. When destination is set to last, the last coordinate will be used 
//! as destination of the trip in the returned output. If you specify any, any of the 
//! coordinates can be used as the first or last coordinate in the output.
//! 
//! However, if source=any&destination=any the returned round-trip will still start at 
//! the first input coordinate by default.
//! 
//! Currently, not all combinations of roundtrip, source and destination are supported. 
//! Right now, the following combinations are possible:
//! roundtrip | source  | destination | supported
//! ----------+---------+-------------+-----------
//! true      | first   | last        |    yes
//! true      | first   | any         |    yes
//! true      | any     | last        |    yes
//! true      | any     | any         |    yes
//! false     | first   | last        |    yes
//! false     | first   | any         |    no
//! false     | any     | last        |    no
//! false     | any     | any         |    no

use displaythis::Display;
use serde::{Serialize, Deserialize};

use crate::*;

request!(TripRequest (Service::Trip) -> TripResponse {
    /// Returned route is a roundtrip (route returns to first location)
    roundtrip: bool,
    /// Returned route starts at any or first coordinate (by default, any)
    source: Option<Source>,
    /// Returned route ends at any or last coordinate (by default, any)
    destination: Option<Destination>,
    /// Returned route steps for each route
    #[builder(default)]
    steps: bool,
    /// Returned route geometry format (influences overview and per step)
    #[builder(default, setter(into, strip_option))]
    geometries: Option<Geometries>,
    /// Returns additional metadata for each coordinate along the route geometry.
    #[builder(default, setter(into, strip_option))]
    annotations: Option<RouteAnnotationRequest>,
    /// Add overview geometry either full, simplified according to highest zoom level it could be display on, or not at all.
    #[builder(default, setter(into, strip_option))]
    overview: Option<OverviewRequest>
});

impl WithOptions for TripRequest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];
        add_option!(          opts, roundtrip,         self.roundtrip);
        add_option!(opt       opts, source,            self.source);
        add_option!(opt       opts, destination,       self.destination);
        add_option!(          opts, steps,             self.steps);
        add_option!(opt       opts, geometries,        self.geometries);
        add_option!(opt       opts, annotations,       self.annotations);
        add_option!(opt       opts, overview,          self.overview);
        opts
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripResponse {
    /// Array of Waypoint objects representing all waypoints in input order.
    pub waypoints: Option<Vec<TripWaypoint>>,
    /// An array of Route objects that assemble the trace
    pub trips: Option<Vec<Route>>,
}

/// Returned route starts at any or first coordinate (by default, any)
#[derive(Debug, Clone, Copy, Display)]
pub enum Source {
    #[display("first")]
    First,
    #[display("any")]
    Any
}
impl Default for Source {
    fn default() -> Self { Self::Any }
}

/// Returned route ends at any or first coordinate (by default, any)
#[derive(Debug, Clone, Copy, Display)]
pub enum Destination {
    #[display("last")]
    Last,
    #[display("any")]
    Any
}
impl Default for Destination {
    fn default() -> Self { Self::Any }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripWaypoint {
    #[serde(flatten)]
    /// The actual waypoint data
    pub waypoint: Waypoint,
    /// Index to trips of the sub-trip the point was matched to.
    pub trips_index: usize,
    /// Index of the point in the trip.
    pub waypoint_index: usize,
}


#[cfg(test)]
mod tests {
    use crate::{Response, TripResponse};

    #[test]
    fn it_can_parse_respone_with_geojson_geometry() {
        let response = r#"
        {"code":"Ok","trips":[{"geometry":{"coordinates":[[-1.301955,44.189087],[-1.007841,44.051348],[-1.231357,43.695456],[-1.412508,43.614398],[-1.451111,43.476322],[-1.91008,43.284461],[-2.354323,43.286191],[-2.55292,43.167382],[-2.724243,43.215122],[-2.667489,43.319578],[-2.752728,43.455001],[-2.676238,43.337642],[-2.72282,43.211094],[-2.545696,43.16779],[-2.354599,43.286042],[-1.967763,43.276825],[-1.727254,43.34209],[-1.448694,43.477688],[-1.411259,43.615479],[-1.230196,43.696527],[-0.818919,44.402437],[-0.837596,44.647112],[-0.543676,44.781843],[-0.427936,44.984696],[-0.672117,45.755834],[-0.511181,46.167936],[-0.311755,46.340182],[0.282502,46.553937],[0.509817,46.79267],[0.769707,47.470206],[1.135315,47.610263],[1.34018,47.621405],[1.847917,47.904027],[1.905619,48.54117],[2.317712,48.736426],[2.604969,49.180265],[2.453389,49.291287],[2.392425,49.530347],[2.453234,49.291306],[2.604782,49.180304],[2.473611,48.8869],[2.314607,48.734866],[1.905434,48.541184],[1.848262,47.90468],[1.340097,47.62151],[1.135256,47.610375],[0.769579,47.470284],[0.509671,46.792706],[0.282371,46.55399],[-0.309737,46.341282],[-0.506001,46.175304],[-0.672287,45.755833],[-0.42851,44.983539],[-0.544118,44.78165],[-0.837155,44.648282],[-0.818661,44.403963],[-0.961457,44.15121],[-1.301955,44.189087]],"type":"LineString"},"legs":[{"steps":[],"summary":"","weight":13636.7,"duration":13636.7,"distance":276943.4},{"steps":[],"summary":"","weight":40931.4,"duration":40931.4,"distance":1023854.5},{"steps":[],"summary":"","weight":32574.5,"duration":32484.1,"distance":802331.5}],"weight_name":"routability","weight":87142.6,"duration":87052.200000000,"distance":2103129.4}],"waypoints":[{"waypoint_index":0,"trips_index":0,"hint":"o13piLZd6YjoAAAAAQAAALsBAACVAgAAMUOBQpuQ4z330vVCrcE3Q-gAAAABAAAAuwEAAJUCAACL9AAAPSLs_59FogI9Iuz_n0WiAggA7ws8uUOX","distance":0,"name":"","location":[-1.301955,44.189087]},{"waypoint_index":2,"trips_index":0,"hint":"EihKhKQUxZFNAAAAQwAAALgAAACRAQAAqqoKQxUa60K0q6JDOTEyRE0AAABDAAAAuAAAAJEBAACL9AAAaYEkAOvF8wJogSQA6sXzAgUA3xM8uUOX","distance":0.132708606,"name":"","location":[2.392425,49.530347]},{"waypoint_index":1,"trips_index":0,"hint":"XnDchWFz3IUAAAAAEgAAAAAAAAAhAQAAKrSlPYlln0EAAAAA3rygQwAAAAASAAAAAAAAACEBAACL9AAAKP_V_xkSlwIn_9X_GBKXAgAAjwc8uUOX","distance":0.137441217,"name":"","location":[-2.752728,43.455001]}]}
        "#;

        let parsed = serde_json::from_str::<Response<TripResponse>>(response);
        assert!(parsed.is_ok())
    }

    #[test]
    fn it_can_parse_respone_with_string_geometry() {
        let response = r#"
        {"code":"Ok","trips":[{"geometry":"sucuH}ttRdw@gxF~`MhuY`qXuN|dVdp[vnRlVlsYldWbrj@{nDr|oBtpc@bs`@xIfe[|sYhnn@vtO`zMvbWxxMb|ElmVdxd@hg]~joBxsNh|JxrPzyi@f`RttvA`rUzhaAlwD`k}@pi]nas@pt[hiY|oi@tlSntCtm]`vRvk@hsKhaPdvPtyD`|Bv|W|ic@raPtrCrsEaaIxvMfiAswMk|Loa^otIub{A}gI_lYwgNkyM~`Jmff@o{Ba{SqtL{aRjuGkx^uEo__@cyP}|@ovNgzL{nYkeeA{fAqag@mcr@y{mAguBcvRy}Y~Su}Mco_AzuEu`u@mkAy|VbiGsbX{oAiwRljEwf_@o`C{wXpuZoee@hdKg`{@v_Xa{\\bmL{l[`kZslQncLm`y@zsUg{c@`dLmwF~}Mul]leWsyQ|`EqyQkyDa{G`dDaxt@{dQ_lh@o{K_qkAkgGisRfkCi~Egy@wp[bmLmzb@zbLsyPbzQl~NeyEarJyzKybCkiSbg^{{D`eUxhA~o[ytC`mFnpGzbRziLfonAqiT`uU__GnfWwsz@{fK_uOf_F_ha@weJiiPbvB{bTnj[wzXfn~AlCjqp@ylKl`\\cxb@|}DepZymH{iUttCctZocVakn@~KerThaSu`Jpdh@euO{gC_bm@xs[awChcXmnWdps@ojYfog@ebLmdA}_|@dd]_tSdpe@qtLieNsbRqNieU}lVksYkuCezIkzSobDb|@","legs":[{"steps":[],"summary":"","weight":23749.6,"duration":23696.8,"distance":615379.5},{"steps":[],"summary":"","weight":29072.4,"duration":29044,"distance":726181.2},{"steps":[],"summary":"","weight":25294.7,"duration":25235.7,"distance":657695.4}],"weight_name":"routability","weight":78116.7,"duration":77976.5,"distance":1999256.1}],"waypoints":[{"waypoint_index":0,"trips_index":0,"hint":"y8gSgyArG4TDAAAAJAIAADIBAABDAAAAhwhZQmMQGENDhalC2QEVQcMAAAAkAgAAMgEAACIAAACL9AAAsTExACdeBwOxMTEAJ14HAwMAHxU8uUOX","distance":0,"name":"","location":[3.223985,50.814503]},{"waypoint_index":1,"trips_index":0,"hint":"aoEihClduYoNAAAAAAAAAGcAAAAAAAAATQMPQQAAAAD0mo9CAAAAAA0AAAAAAAAAZwAAAAAAAACL9AAA-XPx_0g1zgL5c_H_SjXOAgEAzwI8uUOX","distance":0.222344608,"name":"Allée de Riga","location":[-0.953351,47.068488]},{"waypoint_index":2,"trips_index":0,"hint":"BH0PhgZ9D4Y-AAAAAAAAAHwAAAA-AwAArb4sQgAAAACVbqtCLi4QRD4AAAAAAAAAfAAAAD4DAACL9AAAjK1fAB0BzQKMrV8AHQHNAgMAnwc8uUOX","distance":0,"name":"","location":[6.270348,46.989597]}]}
        "#;

        let parsed = serde_json::from_str::<Response<TripResponse>>(response);
        assert!(parsed.is_ok())
    }
}