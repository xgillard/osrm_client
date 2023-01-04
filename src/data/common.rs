//! This module comprises a representation of the data that is shared accross
//! all OSRM services.

use displaythis::Display;
use serde::{Serialize, Deserialize};


/// Mode of transportation
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum TransportationMode {
    /// Travelling by car
    #[display("car")]
    #[serde(rename="car")]
    Car, 
    /// Travelling by bike
    #[display("bike")]
    #[serde(rename="bike")]
    Bike, 
    /// Travelling on bare foot
    #[display("foot")]
    #[serde(rename="foot")]
    Foot,
}

/// Route geometry format (influences overview and per step)
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum Geometries {
    #[display("polyline")]
    #[serde(rename="polyline")]
    Polyline, 
    #[display("polyline6")]
    #[serde(rename="polyline6")]
    Polyline6, 
    #[display("geojson")]
    #[serde(rename="geojson")]
    GeoJson,
}

/// The location of a point anywhere on earth. The order of the fields is
/// longitude, latitude
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
#[display("{longitude},{latitude}")]
pub struct Location{
    pub longitude: f32, 
    pub latitude: f32
}
impl Location {
    pub fn new(longitude: f32, latitude: f32) -> Self {
        Self { longitude, latitude }
    }
}

/// Most services are quite flexible wrt the coordinates they accept:
/// it can either be a single coord, a sequence of coord separated by semicolon,
/// or a polyline (follows Google polyline format) or polyline with precision of 6.
#[derive(Debug, Clone)]
pub enum Coordinates {
    /// One single coordinate
    Single(Location),
    /// A sequence of coordinates in the longitude, latitude form
    Multi(Vec<Location>),
    /// A polyline formatted according to Google polyline format (precision 5)
    Polyline(String),
    /// A polyline formatted according to Google polyline format (precision 6)
    Polyline6(String),
}
impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(c) => write!(f, "{c}"),
            Self::Multi(coord) => {
                for (i, c) in coord.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{c}")?;
                    } else {
                        write!(f, ";{c}")?;
                    }
                }
                Ok(())
            },
            Self::Polyline(s) => write!(f, "polyline({s})"),
            Self::Polyline6(s) => write!(f, "polyline6({s})"),
        }
    }
}

/// Hint from previous request to derive position in street network (base64 encoded)
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[display("{0}")]
pub struct Hint(String);

/// Object used to describe waypoint on a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    /// Name of the street the coordinate snapped to
    pub name: String,
    /// the [longitude, latitude] pair of the snapped coordinate
    pub location: Location,
    /// The distance, in metres, from the input coordinate to the snapped coordinate
    pub distance: f32, 
    /// Unique internal identifier of the segment (ephemeral, not constant over data 
    /// updates) This can be used on subsequent request to significantly speed up the 
    /// query and to connect multiple services. E.g. you can use the hint value obtained 
    /// by the nearest query as hint values for route inputs.
    pub hint: Hint,
    /// Array of OpenStreetMap node ids
    pub nodes: Option<Vec<usize>>,
}

/// An intersection gives a full representation of any cross-way the path passes bay. 
/// For every step, the very first intersection (intersections[0]) corresponds to the 
/// location of the StepManeuver. Further intersections are listed for every cross-way 
/// until the next turn instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intersection {
    /// the [longitude, latitude] pair describing the location of the turn.
    pub location: Location,
    /// A list of bearing values (e.g. [0,90,180,270]) that are available at the intersection.
    /// The bearings describe all available roads at the intersection. Values are between 
    /// 0-359 (0=true north)
    pub bearings: Vec<u16>,
    /// An array of strings signifying the classes (as specified in the profile) of the road
    /// exiting the intersection
    pub classes:  Vec<String>,
    /// A list of entry flags, corresponding in a 1:1 relationship to the bearings. A value 
    /// of true indicates that the respective road could be entered on a valid route. 
    /// false indicates that the turn onto the respective road would violate a restriction.
    pub entry: Vec<bool>,
    /// index into bearings/entry array. Used to calculate the bearing just before the turn.
    /// Namely, the clockwise angle from true north to the direction of travel immediately 
    /// before the maneuver/passing the intersection. Bearings are given relative to the 
    /// intersection. To get the bearing in the direction of driving, the bearing has to be 
    /// rotated by a value of 180. The value is not supplied for depart maneuvers.
    pub in_index: usize,
    /// index into the bearings/entry array. Used to extract the bearing just after the turn. 
    /// Namely, The clockwise angle from true north to the direction of travel immediately 
    /// after the maneuver/passing the intersection. The value is not supplied for arrive 
    /// maneuvers.
    pub out_index: usize,
    /// Array of Lane objects that denote the available turn lanes at the intersection. 
    /// If no lane information is available for an intersection, the lanes property will not 
    /// be present.
    pub lanes: Vec<Lane>
}

/// A Lane represents a turn lane at the corresponding turn location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lane {
    /// A indication (e.g. marking on the road) specifying the turn lane. A road can have multiple 
    /// indications (e.g. an arrow pointing straight and left). The indications are given in an 
    /// array, each containing one of the following types. Further indications might be added on 
    /// without an API version change.
    /// 
    /// ## Documented Values:
    /// * none           -> No dedicated indication is shown.
    /// * uturn          -> An indication signaling the possibility to reverse (i.e. fully bend arrow).
    /// * sharp right    -> An indication indicating a sharp right turn (i.e. strongly bend arrow).
    /// * right          -> An indication indicating a right turn (i.e. bend arrow).
    /// * slight right   -> An indication indicating a slight right turn (i.e. slightly bend arrow).
    /// * straight       -> No dedicated indication is shown (i.e. straight arrow).
    /// * slight left    -> An indication indicating a slight left turn (i.e. slightly bend arrow).
    /// * left           -> An indication indicating a left turn (i.e. bend arrow).
    /// * sharp left     -> An indication indicating a sharp left turn (i.e. strongly bend arrow).
    indications: Vec<DirectionChange>,
    /// a boolean flag indicating whether the lane is a valid choice in the current maneuver
    valid: bool,
}

/// An indication of a change of direction
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum DirectionChange {
    /// An indication signaling the possibility to reverse (i.e. fully bend arrow).
    #[display("uturn")]
    #[serde(rename="uturn")]
    Uturn,
    /// An indication indicating a sharp right turn (i.e. strongly bend arrow).
    #[display("sharp right")]
    #[serde(rename="sharp right")]
    SharpRight,
    /// An indication indicating a right turn (i.e. bend arrow).
    #[display("right")]
    #[serde(rename="right")]
    Right,
    /// An indication indicating a slight right turn (i.e. slightly bend arrow).
    #[display("slight right")]
    #[serde(rename="slight right")]
    SlightRight,
    /// No dedicated indication is shown (i.e. straight arrow).
    #[display("straight")]
    #[serde(rename="straight")]
    Straight,
    /// An indication indicating a slight left turn (i.e. slightly bend arrow).
    #[display("slight left")]
    #[serde(rename="slight left")]
    SlightLeft,
    /// An indication indicating a left turn (i.e. bend arrow).
    #[display("left")]
    #[serde(rename="left")]
    Left,
    /// An indication indicating a sharp left turn (i.e. strongly bend arrow).
    #[display("sharp left")]
    #[serde(rename="sharp left")]
    SharpLeft,
}

/// type A string indicating the type of maneuver. new identifiers might be introduced 
/// without API change Types unknown to the client should be handled like the turn type, 
/// the existence of correct modifier values is guranteed
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum ManeuverType {
    /// a basic turn into direction of the modifier
    #[display("turn")]
    #[serde(rename="turn")]
    Turn,
    /// no turn is taken/possible, but the road name changes
    #[display("new name")]
    #[serde(rename="new name")]
    NewName,
    /// indicates the departure of the leg
    #[display("depart")]
    #[serde(rename="depart")]
    Depart,
    /// indicates the destination of the leg
    #[display("arrive")]
    #[serde(rename="arrive")]
    Arrive,
    /// merge onto a street (e.g. getting on the highway from a ramp, the modifier specifies 
    /// the direction of the merge )
    #[display("merge")]
    #[serde(rename="merge")]
    Merge, 
    /// Deprecated . Replaced by on_ramp and off_ramp .
    #[display("ramp")]
    #[serde(rename="ramp")]
    Ramp,
    /// take a ramp to enter a highway (direction given my modifier )
    #[display("on ramp")]
    #[serde(rename="on ramp")]
    OnRamp,
    /// take a ramp to exit a highway (direction given my modifier )
    #[display("off ramp")]
    #[serde(rename="off ramp")]
    OffRamp,
    /// take the left/right side at a fork depending on modifier
    #[display("fork")]
    #[serde(rename="fork")]
    Fork,
    /// road ends in a T intersection turn in direction of modifier
    #[display("end of road")]
    #[serde(rename="end of road")]
    EnfOfRoad,
    /// Deprecated replaced by lanes on all intersection entries
    #[display("use lane")]
    #[serde(rename="use lane")]
    UseLane,
    /// Turn in direction of modifier to stay on the same road
    #[display("continue")]
    #[serde(rename="continue")]
    Continue,
    /// traverse roundabout, if the route leaves the roundabout there will be
    /// an additional property exit for exit counting. The modifier specifies 
    /// the direction of entering the roundabout.
    #[display("roundabout")]
    #[serde(rename="roundabout")]
    Roundabout,
    /// a traffic circle. While very similar to a larger version of a roundabout, 
    /// it does not necessarily follow roundabout rules for right of way. It can
    /// offer rotary_name and/or rotary_pronunciation parameters (located in the 
    /// RouteStep object) in addition to the exit parameter (located on the StepManeuver 
    /// object).
    #[display("rotary")]
    #[serde(rename="rotary")]
    Rotary,
    /// Describes a turn at a small roundabout that should be treated as normal turn. 
    /// The modifier indicates the turn direciton. 
    /// Example instruction: At the roundabout turn left .
    #[display("roundabout turn")]
    #[serde(rename="roundabout turn")]
    RoundaboutTurn,
    /// not an actual turn but a change in the driving conditions. 
    /// For example the travel mode or classes. If the road takes a turn itself, 
    /// the modifier describes the direction
    #[display("notification")]
    #[serde(rename="notification")]
    Notification,
    /// Describes a maneuver exiting a roundabout (usually preceeded by a roundabout instruction)
    #[display("exit roundabout")]
    #[serde(rename="exit roundabout")]
    ExitRoundabout,
    /// Describes the maneuver exiting a rotary (large named roundabout)
    #[display("exit rotary")]
    #[serde(rename="exit rotary")]
    ExitRotary,
}

/// A maneuver that must be performed to follow a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepManeuver {
    /// A [longitude, latitude] pair describing the location of the turn.
    pub location: Location,
    /// The clockwise angle from true north to the direction of travel immediately 
    /// before the maneuver. Range 0-359.
    pub bearing_before: u16,
    /// The clockwise angle from true north to the direction of travel immediately 
    /// after the maneuver. Range 0-359.
    pub bearing_aftter: u16,
    /// A string indicating the type of maneuver. 
    /// **new identifiers might be introduced without API change** 
    /// Types unknown to the client should be handled like the turn type, the existence 
    /// of correct modifier values is guranteed.
    #[serde(rename="type")]
    pub maneuver_type: ManeuverType,
    /// An optional string indicating the direction change of the maneuver.
    /// 
    /// The list of turns without a modifier is limited to: depart/arrive. 
    /// If the source/target location is close enough to the depart/arrive location, 
    /// no modifier will be given. The meaning depends on the type property.
    /// 
    /// ## Examples
    /// * turn -> modifier indicates the change in direction accomplished by the turn
    /// * depart / arrive -> modifier indicates the position of departure and arrival 
    ///           point in relation to the current direction of travel.
    pub modifier: Option<DirectionChange>,
    /// An optional integer indicating number of the exit to take. The property exists 
    /// for the roundabout / rotary property: Number of the roundabout exit to take. 
    /// If exit is undefined the destination is on the roundabout
    pub exit: Option<u8>,
}

/// A step consists of a maneuver such as a turn or merge, followed by a distance of 
/// travel along a single way to the subsequent step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStep { 
    /// The distance of travel from the maneuver to the subsequent step, in meters.
    pub distance: f32,
    /// The estimated travel time, in seconds
    pub duration: f32,
    /// The unsimplified geometry of the route segment, depending on the geometries parameter.
    pub geometry: Geometry,
    /// The calculated weight of the step.
    pub weight: f32,
    /// The name of the way along which travel proceeds.
    pub name: String,
    /// A reference number or code for the way. Optionally included, 
    /// if ref data is available for the given way.
    #[serde(rename="ref")]
    pub reference: Option<String>,
    /// A string containing an IPA phonetic transcription indicating how to pronounce the name in 
    /// the name property. This property is omitted if pronunciation data is unavailable for the step.
    pub pronunciation: Option<String>,
    // /// The destinations of the way. Will be undefined if there are no destinations
    // pub destinations: Vec< WHAT_SHOULD_I_USE_HERE >,
    // /// The exit numbers or names of the way. Will be undefined if there are no exit numbers or names
    // pub exits: Vec< WHAT_SHOULD_I_USE_HERE >,
    
    /// A string signifying the mode of transportation
    pub mode: TransportationMode,
    /// A StepManeuver object representing the maneuver
    pub maneuver: StepManeuver,
    /// A list of Intersection objects that are passed along the segment, the very first belonging 
    /// to the StepManeuver
    pub intersections: Vec<Intersection>,
    /// The name for the rotary. Optionally included, if the step is a rotary and a rotary name is available
    pub rotary_name: Option<String>,
    /// The pronunciation hint of the rotary name. Optionally included, if the step is a rotary and 
    /// a rotary pronunciation is available.
    pub rotary_pronunciation: Option<String>,
    /// The legal driving side at the location for this step. Either left or right
    pub driving_side: Option<DrivingSide>,
}

/// The legal driving side at a location
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum DrivingSide {
    /// a basic turn into direction of the modifier
    #[display("left")]
    #[serde(rename="left")]
    Left,
    #[display("right")]
    #[serde(rename="right")]
    Right,
}

/// Annotation of the whole route leg with fine-grained information about each segment or node id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// The distance, in metres, between each pair of coordinates
    pub distance: f32,
    /// The duration between each pair of coordinates, in seconds. Does not include the 
    /// duration of any turns
    pub duration: f32,
    /// The index of the datasource for the speed between each pair of coordinates. 0 is the default 
    /// profile, other values are supplied via --segment-speed-file to osrm-contract or osrm-customize. 
    /// String-like names are in the metadata.datasource_names array.
    pub datasources: Vec<usize>,
    /// The OSM node ID for each coordinate along the route, excluding the first/last user-supplied
    /// coordinates
    pub nodes: Option<Vec<usize>>,
    /// The weights between each pair of coordinates. Does not include any turn costs
    pub weight: Vec<f32>,
    /// Convenience field, calculation of distance / duration rounded to one decimal place
    pub speed: f32,
    /// Metadata related to other annotations
    pub metadata: AnnotationMetaData,
}
/// Some meta-data attached to route annotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationMetaData {
    /// The names of the datasources used for the speed between each pair of coordinates. lua profile 
    /// is the default profile, other values arethe filenames supplied via --segment-speed-file to 
    /// osrm-contract or osrm-customize
    pub datasource_names: Option<Vec<String>>,
}
/// Represents a route between two waypoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteLeg {
    /// The distance traveled by this route leg, in float meters.
    pub distance: f32,
    /// The estimated travel time, in of seconds
    pub duration: f32,
    /// The calculated weight of the route leg.
    pub weight: f32,
    /// Summary of the route taken as string. Depends on the summary parameter
    /// * true -> Names of the two major roads used. Can be empty if route is too short
    /// * false-> empty string
    pub summary: String,
    /// Depends on the steps parameter.
    /// * true -> array of RouteStep objects describing the turn-by-turn instructions
    /// * false-> empty array
    pub steps: Vec<RouteStep>,
    /// Additional details about each coordinate along the route geometry
    /// * true -> An Annotation object containing node ids, durations, distances and weights.
    /// * false-> undefined (none)
    pub annotation: Option<Annotation>
}
/// Represents a route through (potentially multiple) waypoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    /// The distance traveled by this route, in meters.
    pub distance: f32,
    /// The estimated travel time, in seconds
    pub duration: f32,
    /// The whole geometry of the route value depending on overview parameter, format depending on 
    /// the geometries parameter. See RouteStep's geometry property for a parameter documentation.
    pub geometry: Geometry,
    /// The calculated weight of the route.
    pub weight: f32,
    /// The name of the weight profile used during extraction phas
    pub weight_name: String,
    /// The legs between the given waypoints, an array of RouteLeg objects.
    pub legs: Vec<RouteLeg>,
}

/// Represents a geometry which can either be encoded with polyline of polyline6
/// or explicit in the form of a geojson
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Geometry {
    /// When the geometry is encoded with polyline or polyline6
    Encoded(String),
    /// When the geometry is explicitly detailed
    Explicit(GeoJsonGeometry)
}

/// GeoJSON[1] is an open standard format designed for representing simple geographical features, 
/// along with their non-spatial attributes. It is based on the JSON format.
/// 
/// The features include points (therefore addresses and locations), line strings (therefore streets, 
/// highways and boundaries), polygons (countries, provinces, tracts of land), and multi-part 
/// collections of these types. GeoJSON features need not represent entities of the physical 
/// world only; mobile routing and navigation apps, for example, might describe their service 
/// coverage using GeoJSON.[2]
///
/// The GeoJSON format differs from other GIS standards in that it was written and is maintained 
/// not by a formal standards organization, but by an Internet working group of developers.[3] 
/// 
/// ## Geometries
/// Points are [x, y] or [x, y, z]. They may be [longitude, latitude] or [eastings, northings]. 
/// Elevation is an optional third number. They are decimal numbers. [6]
/// For example, London (51.5074° North, 0.1278° West) is [-0.1278, 51.5074] 
/// 
/// (Ref: https://en.wikipedia.org/wiki/GeoJSON#Geometries)
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum GeoJsonGeometry {
    Point { coordinates: GeoJsonPoint },
    LineString { coordinates: Vec<GeoJsonPoint> },
    Polygon { coordinates: Vec<Vec<GeoJsonPoint>> },
    MultiPoint { coordinates: Vec<GeoJsonPoint> },
    MultiLineString { coordinates: Vec<Vec<GeoJsonPoint>> },
    MultiPolygon { coordinates: Vec<Vec<Vec<GeoJsonPoint>>> },
}

/// Points are [x, y] or [x, y, z]. They may be [longitude, latitude] or [eastings, northings]. 
/// Elevation is an optional third number. They are decimal numbers. [6]
/// For example, London (51.5074° North, 0.1278° West) is [-0.1278, 51.5074] 
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeoJsonPoint {
    Regular([f32; 2]),
    Elevated([f32; 3]),
}
impl GeoJsonPoint {
    pub fn location(self) -> Location {
        match self {
            GeoJsonPoint::Regular(x)  => Location { longitude: x[0], latitude: x[1] },
            GeoJsonPoint::Elevated(x) => Location { longitude: x[0], latitude: x[1] },
        }
    }
    pub fn elevation(self) -> Option<f32> {
        match self {
            GeoJsonPoint::Regular(_)  => None,
            GeoJsonPoint::Elevated(x) => Some(x[2]),
        }
    }
    pub fn coordinates(&self) -> &[f32] {
        match self {
            GeoJsonPoint::Regular(x)  => x,
            GeoJsonPoint::Elevated(x) => x,
        }
    }
}
impl From<Location> for GeoJsonPoint {
    fn from(Location { longitude, latitude }: Location) -> Self {
        Self::Regular([longitude, latitude])
    }
}