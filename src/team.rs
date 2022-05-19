use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};

pub const TEAMS: [Team; 31] = [
    Team::All,
    Team::AtlantaHawks,
    Team::BostonCeltics,
    Team::BrooklynNets,
    Team::CharlotteHornets,
    Team::ChicagoBulls,
    Team::ClevelandCavaliers,
    Team::DallasMavericks,
    Team::DenverNuggets,
    Team::DetroitPistons,
    Team::GoldenStateWarriors,
    Team::HoustonRockets,
    Team::IndianaPacers,
    Team::LAClippers,
    Team::LosAngelesLakers,
    Team::MemphisGrizzlies,
    Team::MiamiHeat,
    Team::MilwaukeeBucks,
    Team::MinnesotaTimberwolves,
    Team::NewOrleansPelicans,
    Team::NewYorkKnicks,
    Team::OklahomaCityThunder,
    Team::OrlandoMagic,
    Team::Philadelphia76ers,
    Team::PhoenixSuns,
    Team::PortlandTrailBlazers,
    Team::SacramentoKings,
    Team::SanAntoniaSpurs,
    Team::TorontoRaptors,
    Team::UtahJazz,
    Team::WashingtonWizards,
];

#[derive(PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Team {
    All = 0,
    Unknown = 1,
    AtlantaHawks = 1610612737,
    BostonCeltics = 1610612738,
    BrooklynNets = 1610612751,
    CharlotteHornets = 1610612766,
    ChicagoBulls = 1610612741,
    ClevelandCavaliers = 1610612739,
    DallasMavericks = 1610612742,
    DenverNuggets = 1610612743,
    DetroitPistons = 1610612765,
    GoldenStateWarriors = 1610612744,
    HoustonRockets = 1610612745,
    IndianaPacers = 1610612754,
    LAClippers = 1610612746,
    LosAngelesLakers = 1610612747,
    MemphisGrizzlies = 1610612763,
    MiamiHeat = 1610612748,
    MilwaukeeBucks = 1610612749,
    MinnesotaTimberwolves = 1610612750,
    NewOrleansPelicans = 1610612740,
    NewYorkKnicks = 1610612752,
    OklahomaCityThunder = 1610612760,
    OrlandoMagic = 1610612753,
    Philadelphia76ers = 1610612755,
    PhoenixSuns = 1610612756,
    PortlandTrailBlazers = 1610612757,
    SacramentoKings = 1610612758,
    SanAntoniaSpurs = 1610612759,
    TorontoRaptors = 1610612761,
    UtahJazz = 1610612762,
    WashingtonWizards = 1610612764,
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::AtlantaHawks => write!(f, "Atlanta Hawks"),
            Team::BostonCeltics => write!(f, "Boston Celtics"),
            Team::BrooklynNets => write!(f, "Brooklyn Nets"),
            Team::CharlotteHornets => write!(f, "Charlotte Hornets"),
            Team::ChicagoBulls => write!(f, "Chicago Bulls"),
            Team::ClevelandCavaliers => write!(f, "Cleveland Cavaliers"),
            Team::DallasMavericks => write!(f, "Dallas Mavericks"),
            Team::DenverNuggets => write!(f, "Denver Nuggets"),
            Team::DetroitPistons => write!(f, "Detroit Pistons"),
            Team::GoldenStateWarriors => write!(f, "Golden State Warriors"),
            Team::HoustonRockets => write!(f, "Houston Rockets"),
            Team::IndianaPacers => write!(f, "Indiana Pacers"),
            Team::LAClippers => write!(f, "LA Clippers"),
            Team::LosAngelesLakers => write!(f, "Los Angeles Lakers"),
            Team::MemphisGrizzlies => write!(f, "Memphis Grizzlies"),
            Team::MiamiHeat => write!(f, "Miami Heat"),
            Team::MilwaukeeBucks => write!(f, "Milwaukee Bucks"),
            Team::MinnesotaTimberwolves => write!(f, "Minnesota Timberwolves"),
            Team::NewOrleansPelicans => write!(f, "New Orleans Pelicans"),
            Team::NewYorkKnicks => write!(f, "New York Knicks"),
            Team::OklahomaCityThunder => write!(f, "Oklahoma City Thunder"),
            Team::OrlandoMagic => write!(f, "Orlando Magic"),
            Team::Philadelphia76ers => write!(f, "Philadelphia 76ers"),
            Team::PhoenixSuns => write!(f, "Phoenix Suns"),
            Team::PortlandTrailBlazers => write!(f, "Portland Trail Blazers"),
            Team::SacramentoKings => write!(f, "Sacramento Kings"),
            Team::SanAntoniaSpurs => write!(f, "San Antonia Spurs"),
            Team::TorontoRaptors => write!(f, "Toronto Raptors"),
            Team::UtahJazz => write!(f, "Utah Jazz"),
            Team::WashingtonWizards => write!(f, "Washington Wizards"),
            Team::Unknown => write!(f, "Unknown"),
            Team::All => write!(f, "All"),
        }
    }
}