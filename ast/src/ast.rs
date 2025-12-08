use syn::Ident;

// Types
pub type Stage = Ident;
pub type PlayerName = Ident;
pub type TeamName = Ident;
pub type Location = Ident;
pub type Token = Ident;

pub type Precedence = Ident;
pub type PointMap = Ident;
pub type Combo = Ident;
pub type Memory = Ident;
pub type Key = Ident;
pub type Value = Ident;

pub type ID = Ident; 

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
    pub locations: Vec<Location>
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
pub enum CardPosition {
    At(Location, IntExpr),
    Top(Location),
    Bottom(Location),
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
pub enum Quantity {
    Int(IntExpr),
    Quantifier(Quantifier),
    IntRange(IntRange),
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
    Location(Location),
    LocationWhere(Location, FilterExpr),
    LocationCollection(LocationCollection),
    LocationCollectionWhere(LocationCollection, FilterExpr),
    ComboInLocation(Combo, Location),
    ComboInLocationCollection(Combo, LocationCollection),
    NotComboInLocation(Combo, Location),
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
pub struct Game {
    pub flows: Vec<FlowComponent>
}

#[derive(Debug, PartialEq)]
pub enum FlowComponent {
    Stage(SeqStage),
    Rule(Rule),
    IfRule(IfRule),
    ChoiceRule(ChoiceRule),
    OptionalRule(OptionalRule),
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
    CreatePlayer(Vec<PlayerName>),
    CreateTeam(TeamName, Vec<PlayerName>),
    CreateTurnorder(Vec<PlayerName>),
    CreateTurnorderRandom(Vec<PlayerName>),
    CreateLocationOnPlayerCollection(Location, PlayerCollection),
    CreateLocationOnTeamCollection(Location, TeamCollection),
    CreateLocationOnTable(Location),
    CreateLocationCollectionOnPlayerCollection(LocationCollection, PlayerCollection),
    CreateLocationCollectionOnTeamCollection(LocationCollection, TeamCollection),
    CreateLocationCollectionOnTable(LocationCollection),
    CreateCardOnLocation(Location, Types),
    CreateTokenOnLocation(IntExpr, Token, Location),
    CreatePrecedence(Precedence, OnKeyPrec),
    CreatePrecedencePairs(Precedence, KeyValuePairs),
    CreateCombo(Combo, FilterExpr),
    CreateMemoryIntPlayerCollection(Memory, IntExpr, PlayerCollection),
    CreateMemoryStringPlayerCollection(Memory, StringExpr, PlayerCollection),
    CreateMemoryIntTable(Memory, IntExpr),
    CreateMemoryStringTable(Memory, StringExpr),
    CreateMemoryPlayerCollection(Memory, PlayerCollection),
    CreateMemoryTable(Memory),
    CreatePointMap(Precedence, OnKeyPoint),
    CreatePointMapPairs(Precedence, KeyValueInt),
    // Actions
    FlipAction(CardSet, Status),
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
    BidActionMemory(Memory, Quantity),
    EndTurn,
    EndStage,
    EndGameWithWinner(PlayerExpr),
    DemandCardPositionAction(CardPosition),
    DemandStringAction(StringExpr),
    DemandIntAction(IntExpr),
    // Move-Actions
    ClassicMove(ClassicMove),
    DealMove(DealMove),
    ExchangeMove(ExchangeMove),
    TokenMove(TokenMove),
}

#[derive(Debug, PartialEq)]
pub struct Types {
    pub types: Vec<(Key, Vec<Value>)>
}

#[derive(Debug, PartialEq)]
pub struct OnKeyPrec {
    pub key: Key,
    pub values: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct KeyValuePairs {
    pub key_value: Vec<(Key, Value)>
}

#[derive(Debug, PartialEq)]
pub struct ValueIntPair {
    pub value: Value,
    pub int: IntExpr
}

#[derive(Debug, PartialEq)]
pub struct OnKeyPoint {
    pub key: Key,
    pub value_int_vec: Vec<ValueIntPair>,
}

#[derive(Debug, PartialEq)]
pub struct KeyValueInt {
    pub key_value_int_vec: Vec<(Key, Value, IntExpr)>,
}

#[derive(Debug, PartialEq)]
pub struct SeqStage {
    pub stage: Stage,
    pub player: PlayerExpr,
    pub end_condition: EndCondition,
    pub flows: Vec<FlowComponent>,
}

#[derive(Debug, PartialEq)]
pub struct IfRule {
    pub condition: BoolExpr,
    pub flows: Vec<FlowComponent>,
}

#[derive(Debug, PartialEq)]
pub struct OptionalRule {
    pub flows: Vec<FlowComponent>
}

#[derive(Debug, PartialEq)]
pub struct ChoiceRule {
    pub options: Vec<FlowComponent>
}

#[derive(Debug, PartialEq)]
pub enum ClassicMove {
    Move(CardSet, Status, CardSet),
    MoveQuantity(Quantity, CardSet, Status, CardSet),
}

#[derive(Debug, PartialEq)]
pub enum DealMove {
    Deal(CardSet, Status, CardSet),
    DealQuantity(Quantity, CardSet, Status, CardSet),
}

#[derive(Debug, PartialEq)]
pub enum ExchangeMove {
    Exchange(CardSet, Status, CardSet),
    ExchangeQuantity(Quantity, CardSet, Status, CardSet),
}

#[derive(Debug, PartialEq)]
pub enum TokenMove {
    Place(TokenLocExpr, TokenLocExpr),
    PlaceQuantity(Quantity, TokenLocExpr, TokenLocExpr),
}

#[derive(Debug, PartialEq)]
pub enum TokenLocExpr {
    Location(Location),
    LocationCollection(LocationCollection),
    LocationPlayer(Location, PlayerExpr),
    LocationCollectionPlayer(LocationCollection, PlayerExpr),
    LocationPlayerCollection(Location, PlayerCollection),
    LocationCollectionPlayerCollection(LocationCollection, PlayerCollection),
}

#[derive(Debug, PartialEq)]
pub enum ScoreRule {
    ScorePlayer(IntExpr, PlayerExpr),
    ScorePlayerMemory(IntExpr, Memory, PlayerExpr),
    ScorePlayerCollection(IntExpr, PlayerCollection),
    ScorePlayerCollectionMemory(IntExpr, Memory, PlayerCollection),
}

