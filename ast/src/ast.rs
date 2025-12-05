// Types
pub type Stage = String;
pub type PlayerName = String;
pub type TeamName = String;
pub type Location = String;
pub type Token = String;

pub type Precedence = String;
pub type PointMap = String;
pub type Combo = String;
pub type Memory = String;
pub type Key = String;
pub type Value = String;

pub type ID = String; 

// Structs + Enums
#[derive(Debug, PartialEq)]
pub enum PlayerExpr {
    PlayerName(PlayerName),
    Current,
    Next,
    Previous,
    Competitor,
    Turnorder(IntExpr),
    OwnerOf(Box<CardPosition>),
    OwnerOfHighest(Memory),
    OwnerOfLowest(Memory),
}

#[derive(Debug, PartialEq)]
pub enum IntExpr {
    Int(i32),
    IntOp(Box<IntExpr>, Op, Box<IntExpr>),
    IntCollectionAt(Box<IntExpr>),
    SizeOf(Collection),
    SumOfIntCollection(IntCollection),
    SumOfCardSet(Box<CardSet>, PointMap),
    MinOf(Box<CardSet>, PointMap),
    MaxOf(Box<CardSet>, PointMap),
    MinIntCollection(IntCollection),
    MaxIntCollection(IntCollection),
    StageRoundCounter,
    // PlayRoundCounter,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Mod
}

#[derive(Debug, PartialEq)]
pub enum Collection {
    IntCollection(IntCollection),
    StringCollection(StringCollection),
    LocationCollection(LocationCollection),
    PlayerCollection(PlayerCollection),
    TeamCollection(TeamCollection),
    CardSet(Box<CardSet>),
}

#[derive(Debug, PartialEq)]
pub struct IntCollection {
    pub ints: Vec<IntExpr>
}

#[derive(Debug, PartialEq)]
pub struct StringCollection {
    pub strings: Vec<StringExpr>
}

#[derive(Debug, PartialEq)]
pub struct LocationCollection {
    pub locations: Vec<LocationExpr>
}

#[derive(Debug, PartialEq)]
pub enum PlayerCollection {
    Player(Vec<PlayerExpr>),
    Others,
    Quantifier(Quantifier),
    PlayersOut,
    PlayersIn,
}

#[derive(Debug, PartialEq)]
pub enum TeamCollection {
    Team(Vec<TeamExpr>),
    OtherTeams,
}

#[derive(Debug, PartialEq)]
pub enum StringExpr {
    ID(ID),
    KeyOf(Key, CardPosition),
    StringCollectionAt(StringCollection, IntExpr),
}

#[derive(Debug, PartialEq)]
pub enum LocationExpr {
    LocationPlayer(Location, PlayerExpr),
    LocationTeam(Location, TeamExpr),
    LocationCurrentOrTable(Location),
}

#[derive(Debug, PartialEq)]
pub enum CardPosition {
    At(LocationExpr, IntExpr),
    Top(LocationExpr),
    Bottom(LocationExpr),
    MaxPrec (Box<CardSet>, Precedence),
    MinPrec (Box<CardSet>, Precedence),
    MaxPoint(Box<CardSet>, PointMap),
    MinPoint(Box<CardSet>, PointMap),
}

#[derive(Debug, PartialEq)]
pub enum BoolExpr {
    StringEq(StringExpr, StringExpr),
    StringNeq(StringExpr, StringExpr),
    IntCmp(IntExpr, IntCmpOp, IntExpr),
    CardSetEq(CardSet, CardSet),
    CardSetNeq(CardSet, CardSet),
    CardSetIsEmpty(CardSet),
    CardSetIsNotEmpty(CardSet),
    PlayerEq(PlayerExpr, PlayerExpr),
    PlayerNeq(PlayerExpr, PlayerExpr),
    TeamEq(TeamExpr, TeamExpr),
    TeamNeq(TeamExpr, TeamExpr),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    Not(Box<BoolExpr>),
    OutOfStagePlayer(PlayerExpr),
    OutOfGamePlayer(PlayerExpr),
    OutOfStageCollection(PlayerCollection),
    OutOfGameCollection(PlayerCollection),
}

#[derive(Debug, PartialEq)]
pub enum IntCmpOp {
    Eq,
    Neq,
    Gt,
    Lt,
    Ge,
    Le
}

#[derive(Debug, PartialEq)]
pub enum Status {
    FaceUp,
    FaceDown,
    Private
}

#[derive(Debug, PartialEq)]
pub enum TeamExpr {
    TeamName(TeamName),
    TeamOf(PlayerExpr)
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub flows: Vec<FlowComponent>
}

#[derive(Debug, PartialEq)]
pub enum FlowComponent {
    Stage(SeqStage),
    Rule(Rule)
}

#[derive(Debug, PartialEq)]
pub enum EndCondition {
    UntilBool(BoolExpr),
    UntilBoolAndRep(BoolExpr, Repititions),
    UntilBoolOrRep(BoolExpr, Repititions),
    UntilRep(Repititions),
    UntilEnd
}   

#[derive(Debug, PartialEq)]
pub struct Repititions {
    pub times: IntExpr
}

#[derive(Debug, PartialEq)]
pub enum Rule {
    // Creations
    CreatePlayer(Vec<PlayerExpr>),
    CreateTeam(TeamName, Vec<PlayerExpr>),
    CreateTurnorder(PlayerCollection),
    CreateTurnorderRandom(PlayerCollection),
    CreateLocationOnPlayerCollection(PlayerCollection),
    CreateLocationOnTeamCollection(TeamCollection),
    CreateLocationOnTable,
    CreateCardOnLocation(LocationExpr, Vec<(Key, Vec<Value>)>),
    CreateTokenOnLocation(LocationExpr, IntExpr, Token),
    CreatePrecedence(Precedence, Key, Vec<Value>),
    CreatePrecedencePair(Precedence, Vec<(Key, Value)>),
    CreateCombo(Combo, FilterExpr),
    CreateMemoryIntPlayerCollection(Memory, IntExpr, PlayerCollection),
    CreateMemoryStringPlayerCollection(Memory, StringExpr, PlayerCollection),
    CreateMemoryIntTable(Memory, IntExpr, PlayerCollection),
    CreateMemoryStringTable(Memory, StringExpr),
    CreatePointMap(Precedence, Key, Vec<(Value, IntExpr)>),
    CreatePointMapPair(Precedence, Vec<(Key, (Value, IntExpr))>),
    // Actions
    FlipAction(CardSet),
    ShuffleAction(CardSet),
    PlayerOutOfStageAction(PlayerExpr),
    PlayerOutOfGameSuccAction(PlayerExpr),
    PlayerOutOfGameFailAction(PlayerExpr),
    PlayerCollectionOutOfStageAction(PlayerCollection),
    PlayerCollectionOutOfGameSuccAction(PlayerCollection),
    PlayerCollectionOutOfGameFailAction(PlayerCollection),
    SetMemoryInt(Memory, IntExpr),
    SetMemoryString(Memory, StringExpr),
    SetMemoryCollection(Memory, Collection),
    CycleAction(PlayerExpr),
    BidAction(Quantity),
    BidActionMemory(Quantity),
    EndTurn,
    EndStage,
    EndGameWithWinner(PlayerExpr),
    DemandCardPositionAction(CardPosition),
    DemandStringAction(StringExpr),
    DemandIntAction(IntExpr),
}

#[derive(Debug, PartialEq)]
pub struct SeqStage {
    id: String,
    player: PlayerExpr,
    end_condition: EndCondition,
    flows: Vec<FlowComponent>,
}

#[derive(Debug, PartialEq)]
pub struct IfRule {
    condition: BoolExpr,
    flows: Vec<FlowComponent>,
}

#[derive(Debug, PartialEq)]
pub struct OptionalRule {
    flows: Vec<FlowComponent>
}

#[derive(Debug, PartialEq)]
pub struct ChoiceRule {
    options: Vec<FlowComponent>
}

#[derive(Debug, PartialEq)]
pub enum Quantity {
    Int(IntExpr),
    Quantifier,
    IntRange,
}

#[derive(Debug, PartialEq)]
pub enum IntRange {
    Eq(IntExpr),
    Neq(IntExpr),
    Gt(IntExpr),
    Lt(IntExpr),
    Ge(IntExpr),
    Le(IntExpr),
}

#[derive(Debug, PartialEq)]
pub enum Quantifier {
    All,
    Any
}

#[derive(Debug, PartialEq)]
pub enum CardSet {
    Group(Group),
    GroupOfPlayer(Group, PlayerExpr),
    GroupOfPlayerCollection(Group, PlayerCollection),
}

#[derive(Debug, PartialEq)]
pub enum Group {
    Location(LocationExpr),
    LocationWhere(LocationExpr, FilterExpr),
    LocationCollection(LocationCollection),
    LocationCollectionWhere(LocationCollection, FilterExpr),
    ComboInLocation(Combo, LocationExpr),
    ComboInLocationCollection(Combo, LocationCollection),
    NotComboInLocation(Combo, LocationExpr),
    NotComboInLocationCollection(Combo, LocationCollection),
    CardPosition(CardPosition),
}

#[derive(Debug, PartialEq)]
pub enum FilterExpr {
    Same(Key),
    Distinct(Key),
    Adjacent(Key, Precedence),
    Higher(Key, Precedence),
    Lower(Key, Precedence),
    SizeEq (Box<IntExpr>),
    SizeNeq(Box<IntExpr>),
    SizeGt (Box<IntExpr>),
    SizeLt (Box<IntExpr>),
    SizeGe (Box<IntExpr>),
    SizeLe (Box<IntExpr>),
    KeyEq  (Key, Box<StringExpr>),
    KeyNeq (Key, Box<StringExpr>),
    NotCombo(Combo),
    Combo(Combo),
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
}

#[derive(Debug, PartialEq)]
pub enum ClassicMove {
    Move(CardSet, Status, CardSet),
    MoveQuantifier(CardSet, Status, CardSet),
}

#[derive(Debug, PartialEq)]
pub enum DealMove {
    Deal(CardSet, Status, CardSet),
    DealQuantifier(CardSet, Status, CardSet),
}

#[derive(Debug, PartialEq)]
pub enum ExchangeMove {
    Exchange(CardSet, Status, CardSet),
    ExchangeQuantifier(CardSet, Status, CardSet),
}

#[derive(Debug, PartialEq)]
pub enum TokeMove {
    Place(TokenLocExpr, TokenLocExpr),
    PlaceQuantifier(TokenLocExpr, Status, TokenLocExpr),
}

#[derive(Debug, PartialEq)]
pub enum TokenLocExpr {
    LocationPlayer(LocationExpr),
    LocationCollectionPlayer(LocationCollection),
    LocationPlayerCollection(LocationExpr),
    LocationCollectionPlayerCollection(LocationCollection),
}

#[derive(Debug, PartialEq)]
pub enum ScoreRule {
    ScorePlayer(IntExpr, PlayerExpr),
    ScorePlayerMemory(IntExpr, Memory, PlayerExpr),
    ScorePlayerCollection(IntExpr, PlayerCollection),
    ScorePlayerCollectionMemory(IntExpr, Memory, PlayerCollection),
}
