/*
    You can define more AST-structures in here.
    It will automatically generate a Spanned-Version of the new AST
    and the corresponding Lowering/Walking logic for it.

    Be careful with naming certain structs and enums:
    For example the Spanned versions have always a 'S' at the front
    of the original naming:
    IntExpr -> SIntExpr

    Do not do naming like this (in this file):
    enum Template {
        ...
    } 

    enum STemplate {
        ...
    }

    This will cause an error.

    For more information look at code_gen.
*/

use code_gen::*;

#[spanned_ast]
pub mod ast {
    use arbitrary::Arbitrary;
    use serde::{Serialize, Deserialize};
    // For generating Arbitrary ASTs (for Testing)
    use crate::arbitrary::{
        gen_vec_min_1,
        gen_team_name, 
        gen_vec_strings, 
        gen_vec_players_prefixed, 
        gen_ident, 
        gen_vec_teams_with_players,
        gen_types_and_subtypes,
        gen_vec_min_1_kvs,
        gen_vec_min_1_kvis,
        gen_vec_min_1_ints,
        gen_flows_safe,
    };

    // Operator
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================
    /// Arithmetic operators for integer operations.
    /// 
    /// These map directly to standard mathematical symbols used in expressions
    /// like `1 + 1` or `x * y`.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum IntOp {
        /// Addition (`+`)
        Plus,
        /// Subtraction (`-`)
        Minus,
        /// Multiplication (`*`)
        Mul,
        /// Division (`/`)
        Div,
        /// Modulo/Remainder (`%`)
        Mod,
    }

    /// Comparison operators for integer values.
    ///
    /// These operators are used in conditional expressions to compare two 
    /// integers and return a boolean result.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum IntCompare {
        /// Equality (`==`)
        Eq,
        /// Inequality (`!=`)
        Neq,
        /// Greater than (`>`)
        Gt,
        /// Less than (`<`)
        Lt,
        /// Greater than or equal to (`>=`)
        Ge,
        /// Less than or equal to (`<=`)
        Le,
    }
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    // Utility
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================
    /// Stores the name of the memory and optionally its owner.
    /// 
    /// UseMemory is used for Collection.
    /// # Example
    /// ```text
    /// IntCollection
    /// ```
    /// 
    /// These Collections are allowed to have an arbitrary 'Owner'.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum UseMemory {
        /// Single Memory-Name (without owner) 
        Memory { 
            #[arbitrary(with = gen_ident)]
            memory: String
        },
        /// Memory-Name with corresponding Owner 
        WithOwner { 
            #[arbitrary(with = gen_ident)]
            memory: String,
            owner: Box<Owner>
        },
    }

    /// Stores the name of the memory and optionally its (single) owner.
    /// 
    /// UseSingleMemory is used for 'Expr'.
    /// # Example
    /// ```text
    /// PlayerExpr
    /// ```
    /// 
    /// The difference to 'UseMemory' is that it allows only single owners.
    /// If it would allow multiple owners (e.g. PlayCollection) it would have
    /// a different semantic meaning.
    /// # Example
    /// ```text
    /// &(I:Bid of all)
    /// ```
    /// 
    /// => This is an IntCollection and not a IntExpr!
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum UseSingleMemory {
        /// Single Memory-Name (without owner) 
        Memory { 
            #[arbitrary(with = gen_ident)]
            memory: String
        },
        /// Single Memory-Name with 'SingleOwner' 
        WithOwner { 
            #[arbitrary(with = gen_ident)]
            memory: String,
            owner: Box<SingleOwner>
        },
    }

    /// Keyword for filtering the highest/maximum or lowest/minimum of
    /// a Collection or CardSet potentially using a Precedence/PointMap.
    /// 
    /// # Example
    /// ```text
    /// max of ExampleCardSet using ExamplePrecedence
    /// ```
    /// 
    /// You can switch out 'max' with 'highest' and 'min' with lowest
    /// because they have the exact same semantic meaning and having two
    /// Extrema-Enums that do the same thing is unnecessary.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Extrema {
        /// Minimum/Lowest
        Min,
        /// Maximum/Highest
        Max,
    }

    /// There are certain 'Game-Structures' where a Players/Teams can be set
    /// out of. These structures are CurrentStage, a specific Stage and Game
    /// and are defined by the enum OutOf
    /// 
    /// # Example
    /// ```text
    /// set current out of stage
    /// set current out of ExampleStage
    /// set current out of game
    /// set current out of game fail
    /// set current out of game successful
    /// ```
    /// 
    /// The Example sets the current Player out of the Current Stage,
    /// specific Stage and Game.
    /// 'out of game' and 'out of game fail' should have the same semantic
    /// meaning for 'OutAction'.
    /// 
    /// 'out of game' and 'out of game fail' have different semantic
    /// meaning for 'BoolExpr':
    /// 'out of game' means all players that are out of game fail or successful.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum OutOf {
        /// Current Stage (the 'deepest' stage in which you are at the moment)
        CurrentStage,
        /// Specific Stage
        Stage { 
            #[arbitrary(with = gen_ident)]
            name: String 
        },
        /// Has different Semantic Meaning depending on how it is used.
        Game,
        /// Player is successfully out of the game (won)
        GameSuccessful,
        /// Player failed and is out of the game (lost)
        GameFail,
    }

    /// Groupable is a wrapper for Location and LocationCollection.
    /// Most of the time you have the option between using a Location
    /// or using a LocationCollection.
    /// Instead of separating each case in each rule we sum it up into one enum.
    /// # Example
    /// ```text
    /// move 1 from ExampleLocation to ExampleLocation
    /// move 1 from ( Loc1, Loc2 ) to ExampleLocation
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Groupable {
        /// Single Location
        Location { 
            #[arbitrary(with = gen_ident)]
            name: String
        },
        /// Multiple Locations
        LocationCollection { location_collection: LocationCollection },
    }

    /// SingleOwner is only used for UseSingleMemory at the moment.
    /// # Example
    /// ```text
    /// &(I:IntMemory of current)
    /// ```
    /// 
    /// If we would use '&(I:IntMemory of all)' it would have a different meaning 
    /// (IntCollection and not IntExpr).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum SingleOwner {
        /// Onwer is Player
        Player { player: PlayerExpr },
        /// Onwer is Team
        Team{ team: TeamExpr},
        /// Onwer is Table
        Table,
    }

    /// MultiOwner is used for describing a rule in any Collection ('AggregateMemory').
    /// # Example
    /// ```text
    /// &(I:IntMemory of all)
    /// ```
    /// Even though we use the Memory of an Int the semantic meaning is an IntCollection.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum MultiOwner {
        /// Multiple Player 
        PlayerCollection { player_collection: Box<PlayerCollection>},
        /// Multiple Teams
        TeamCollection { team_collection: Box<TeamCollection> },
    }



    /// Owner is a wrapper for all things that can own Locations, CardSet, Memory, etc.
    /// Instead of separating each case in each rule we sum it up into one enum.
    /// # Example
    /// ```text
    /// location ExampleLocation on current
    /// location ExampleLocation on T:Team1
    /// location ExampleLocation on ( current,  next )
    /// location ExampleLocation on ( T:Team1,  team of next )
    /// location ExampleLocation on table
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Owner {
        /// Player
        Player { player: PlayerExpr },
        /// Team
        Team{ team: TeamExpr},
        /// Table
        Table,
        /// Players
        PlayerCollection { player_collection: PlayerCollection},
        /// Teams
        TeamCollection {team_collection: TeamCollection},
    }

    /// We need to specify how much needs to be, for example, moved from one place to another.
    /// There are multiple ways:
    /// - A fixed number (IntExpr)
    /// - A qunatifier (all/any)
    /// - A range that needs to be satisfied (e.g. >= 3)
    /// 
    /// # Example
    /// ```text
    /// move all from ExampleLocation to ExampleLocation1
    /// move >= 3 from ExampleLocation to ExampleLocation1
    /// move 3 from ExampleLocation to ExampleLocation1
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Quantity {
        /// Int
        Int {int: IntExpr},
        /// Quantifier
        Quantifier {qunatifier: Quantifier},
        /// IntRange
        IntRange {int_range: IntRange},
    }

    /// We need to specify ranges even more clearly.
    /// To do so we combine ranges to get the range description we want.
    /// For this we need an Range-Operator.
    /// 
    /// # Example
    /// ```text
    /// >= 3 and <= 10
    /// ```
    /// 
    /// There is not specified which operator binds stronger and there are also no '(' ')' given
    /// to give a specific range. -> If needed then implement it.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum IntRangeOperator {
        /// And (Used like Bool-And)
        And,
        /// Or (Used like Bool-Or)
        Or
    }

    /// We need to specify ranges even more clearly.
    /// To do so we combine ranges to get the range description we want.
    /// 
    /// # Example
    /// ```text
    /// move >= 3 and <= 10 from ExampleLocation to ExampleLocation1
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct IntRange {
        pub start: (IntCompare, IntExpr),
        #[arbitrary(with = gen_vec_min_1)]
        pub op_int: Vec<(IntRangeOperator, IntCompare, IntExpr)>,
    }

    /// Quantifier are used for two things at the moment:
    /// - PlayerCollection
    /// - Quantity
    /// # Example
    /// ```text
    /// turnorder all random
    /// move all from Hand to Garbage
    /// ```
    /// 
    /// It is possible to extend this to other Collections and/or Rules.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Quantifier {
        /// All
        All,
        /// Any
        Any,
    }

    /// A 'Stage' follows specific End-Condition.
    /// You could see 'SeqStage' as a while-loop and the
    /// EndCondition as its 'breaking-condition'
    ///  
    /// # Example
    /// ```text
    /// stage ExampleStage for current until Hand empty or 3 times {
    ///     ...
    /// }
    /// ```
    /// 
    /// There are two types of End-Conditions:
    /// - **Bool**: breaks after a condition is not fullfilled
    /// - **Repetitions**: breaks if a certain number of iterations has been reached.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum EndCondition {
        /// Break with Bool
        UntilBool {bool_expr: BoolExpr},
        /// Break with Bool and/or Repititions
        UntilBoolRep {bool_expr: BoolExpr, logic: BoolOp, reps: Repititions},
        /// Break with Repetitions
        UntilRep {reps: Repititions},
        /// Do not break
        UntilEnd,
    }

    /// Repetitions is Part of an EndCondition.
    /// It works on how many Iterations a Stage has done to this point.
    /// 
    /// # Example
    /// ```text
    /// stage ExampleStage for current 3 times {
    ///     ...
    /// }
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct Repititions {
        /// Int
        pub times: IntExpr,
    }

    /// There are certain types a memory can reference.
    /// This is basically variable assignment.
    /// The Memory type is used when a Memory is set and/or initialized.
    /// 
    /// # Example
    /// ```text
    /// memory ExampleMemory 3 on current
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum MemoryType {
        /// Int-Memory
        Int {int: IntExpr},
        /// Player-Memory
        Player {player: PlayerExpr},
        /// Team-Memory
        Team {team: TeamExpr},
        /// String-Memory
        String {string: StringExpr },
        /// PlayerCollection-Memory
        PlayerCollection { players: PlayerCollection},
        /// StringCollection-Memory
        StringCollection { strings: StringCollection},
        /// TeamCollection-Memory
        TeamCollection { teams: TeamCollection},
        /// IntCollection-Memory
        IntCollection { ints: IntCollection},
        /// LocationCollection-Memory
        LocationCollection { locations: LocationCollection},
        /// CardSet-Memory
        CardSet { card_set: CardSet },
    }

    /// Players is a wrapper for PlayerExpr and PlayerCollection.
    /// Instead of separating each case in each rule we sum it up into one enum.
    /// In most rules you only specify for PlayerExpr or PlayerCollection.
    /// 
    /// # Example
    /// ```text
    /// end game with winner current
    /// end game with winner ( P:Player1, P:Player2 )
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Players {
        /// Player
        Player { player: PlayerExpr},
        /// PlayerCollection
        PlayerCollection {player_collection: PlayerCollection},
    }

    /// There are certain things to end in a game:
    /// - **Turn**: Ending a turn of the current Player
    /// - **Current-Stage**: Ending the Current-Stage for everyone
    /// - **Stage**: Ending a specific Stage for everyone
    /// - **Game**: Ending the Game with a specific Player or PlayerCollection
    /// 
    /// # Example
    /// ```text
    /// end turn 
    /// end stage
    /// end ExampleStage
    /// end game with winner ( P:Player1, P:Player2 )
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum EndType {
        /// Turn of the current Player
        Turn,
        /// Current Stage
        CurrentStage,
        /// A specific Stage
        Stage { 
            #[arbitrary(with = gen_ident)]
            stage: String
        },
        /// Game with a specific set of Winners
        GameWithWinner {players: Players},
    }

    /// Certain information is sometimes required of a player.
    /// For example: What Suite is at the top of your Hand?
    /// 
    /// We give a set of types that can be demanded from a Player:
    /// - **CardPosition**
    /// - **String**
    /// - **Int**
    /// 
    /// # Example
    /// ```text
    /// demand top(Hand) of current
    /// demand Suite of top(Stock)
    /// demand &I:ScoreMemory of current
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum DemandType {
        /// Demanding a CardPosition
        CardPosition {card_position: CardPosition},
        /// Demanding a StringExpr
        String {string: StringExpr},
        /// Demanding a IntExpr
        Int{ int: IntExpr},
    }

    /// A Card can have multiple Attributes/Types:
    /// - Rank
    /// - Suite
    /// - ...
    /// 
    /// This is a wrapper to define a set of Card (Types).
    /// 
    /// # Example
    /// ```text
    /// Rank(Ace, Two, Three, Four)
    /// for Suite(Clubs, Spades, Hearts, Diamonds)
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct Types {
        /// The types are sorted by Vec < (Key) -> (Values) >:
        /// (Key) -> (Values) for (Key) -> (Values) for ...
        #[arbitrary(with = gen_types_and_subtypes)]
        pub types: Vec<(String, Vec<String>)>,
    }

    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    // Base Types
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    // Player
    // ===========================================================================
    /// A Player can be defined at runtime.
    /// 
    /// # Example
    /// ```text
    /// current
    /// next
    /// previous
    /// competitor
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum RuntimePlayer {
        /// The Player that is currently playing
        Current,
        /// The Player after curret in the Turn-Order
        Next,
        /// The Player before curret in the Turn-Order
        Previous,
        /// 'SimStage' related.
        Competitor,
    }

    /// A Player being queried from a PlayerCollection or the Turn-Order.
    /// 
    /// # Example
    /// ```text
    /// turnorder[3]
    /// &PC:PlayerColMemory[0]
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum QueryPlayer {
        /// Turn-Order in the Game
        Turnorder {int: IntExpr},
        /// Element of PlayerCollection at Index
        CollectionAt { players: PlayerCollection, int: IntExpr },
    }

    /// A Player being aggregated from the current State.
    /// 
    /// # Example
    /// ```text
    /// owner of max Hand using points ExamplePointMap
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregatePlayer {
        /// Owner of a specific CardPosition
        OwnerOfCardPostion {card_position: Box<CardPosition>},
        /// Owner of highest/lowest Memory
        OwnerOfMemory {
            extrema: Extrema, 
            #[arbitrary(with = gen_ident)]
            memory: String 
        },
    }

    /// Player.
    /// 
    /// # Example
    /// ```text
    /// P:Player1
    /// current
    /// owner of max Hand using points ExamplePointMap
    /// &P:PlayerMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum PlayerExpr {
        /// Name/Identifier of a Player
        Literal { 
            name: String 
        },
        /// Runtime-Keyword
        Runtime {runtime: RuntimePlayer},
        /// Owner of ...
        Aggregate {aggregate: AggregatePlayer},
        /// At a specific position of a PlayerCollection or Turn-Order.
        Query {query: QueryPlayer},
        /// Referencing a Player-Memory
        Memory { memory: UseSingleMemory },
    }
    // ===========================================================================

    // IntExpr
    // ===========================================================================
    /// An IntExpr being queried from a IntCollection.
    /// 
    /// # Example
    /// ```text
    /// ( 1, 2, 3, 4 )[0]
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum QueryInt {
        /// Element at Index of IntCollection
        IntCollectionAt { int_collection: Box<IntCollection>, int_expr: Box<IntExpr> },
    }

    /// An IntExpr aggregated from the current state of the game.
    /// 
    /// # Example
    /// ```text
    /// size of Hand
    /// sum( ( 1, 2, 3 ) )
    /// sum of Hand using ExamplePointMap
    /// max of Hand using ExamplePointMap
    /// max of ( 1, 2, 3 )
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregateInt {
        /// Size of a Collection e.g. CardSet, PlayerCollection
        SizeOf {collection: Collection},
        /// Sum of an IntCollection
        SumOfIntCollection {int_collection: IntCollection},
        /// Sum of a CardSet using a PointMap
        SumOfCardSet{ 
            card_set:Box<CardSet>, 
            #[arbitrary(with = gen_ident)]
            pointmap: String 
        },
        /// Extrema of a CardSet using a PointMap
        ExtremaCardset {
            extrema: Extrema, 
            card_set: Box<CardSet>, 
            #[arbitrary(with = gen_ident)]
            pointmap: String 
        },
        /// Extrema of an IntCollection
        ExtremaIntCollection {extrema: Extrema, int_collection: IntCollection},
    }

    /// An IntExpr that can be defined at runtime.
    /// 
    /// # Example
    /// ```text
    /// stageroundcounter
    /// stageroundcounter(ExampleStage)
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum RuntimeInt {
        /// Round-Counter of the Current Stage
        CurrentStageRoundCounter,
        /// Round-Counter of a specific Stage
        StageRoundCounter { 
            #[arbitrary(with = gen_ident)]
            stage: String 
        },
    }

    /// IntExpr.
    /// 
    /// # Example
    /// ```text
    /// 1
    /// ( 1 + 1 )
    /// ( 1, 2, 3, 4 )[0]
    /// stageroundcounter
    /// &I:ExampleIntMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum IntExpr {
        /// i32
        Literal { int: i32},
        /// Binary operation on two IntExpr
        Binary {int: Box<IntExpr>, op: IntOp, int1: Box<IntExpr>},
        /// Query Int from the current state
        Query {query: QueryInt},
        /// Aggregate Int from the current state
        Aggregate {aggregate: AggregateInt},
        /// Int from the current runtime.
        Runtime {runtime: RuntimeInt },
        /// Memory
        Memory { 
            memory: UseSingleMemory,
        },
    }
    // ===========================================================================

    // String
    // ===========================================================================

    /// A Key of a CardPosition or an Element at an Index of a StringCollection.
    /// 
    /// # Example
    /// ```text
    /// Suite of top(Hand)
    /// ( "Clubs", "Hearts" )[0]
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum QueryString {
        /// Key of a CardPosition
        KeyOf{ 
            #[arbitrary(with = gen_ident)]
            key: String, 
            card_position:CardPosition
        },
        /// Element at an Index of a StringCollection
        StringCollectionAt { string_collection: StringCollection, int_expr: IntExpr },
    }

    /// StirngExpr.
    /// 
    /// # Example
    /// ```text
    /// "Ace"
    /// Suite of top(Hand)
    /// ( "Clubs", "Hearts" )[0]
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum StringExpr {
        /// A Value for Example 'Ace' in Rank(Ace, Two, ...)
        Literal { value: String },
        /// Query of a current game state
        Query { query: QueryString},
        /// Memory
        Memory { memory: UseSingleMemory },
    }
    // ===========================================================================

    // Bool
    // ===========================================================================
    /// Operator for Comparing CardSet
    /// 
    /// # Example
    /// ```text
    /// Hand == Stock
    /// Hand != Stock
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum CardSetCompare {
        /// ==
        Eq,
        /// !=
        Neq,
    }

    /// Operator for Comparing StringExpr
    /// 
    /// # Example
    /// ```text
    /// "Ace" == "Ace"
    /// "Ace" != "Ace"
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum StringCompare {
        /// ==
        Eq,
        /// !=
        Neq,
    }

    /// Operator for Comparing PlayerExpr
    /// 
    /// # Example
    /// ```text
    /// current == next
    /// next != previous
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum PlayerCompare {
        /// ==
        Eq,
        /// !=
        Neq,
    }

    /// Operator for Comparing TeamExpr
    /// 
    /// # Example
    /// ```text
    /// T:T1 == T:T2
    /// T:T1 != T:T2
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum TeamCompare {
        /// ==
        Eq,
        /// !=
        Neq,
    }

    /// (Binary-)Operator for BoolExpr
    /// 
    /// # Example
    /// ```text
    /// and
    /// or
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum BoolOp {
        /// Bool-And
        And,
        /// Bool-Or
        Or,
    }

    /// Unary-Operator for BoolExpr
    /// 
    /// # Example
    /// ```text
    /// not
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum UnaryOp {
        /// Bool-Not
        Not,
    }

    /// All Comparisons that return to a Bool.
    /// 
    /// # Example
    /// ```text
    /// 1 != 2
    /// Hand != Stock
    /// "Ace" != "King"
    /// P:P1 != P:P2
    /// T:T1 != T:T2
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum CompareBool {
        /// Int-Comparison
        Int { int: IntExpr, cmp: IntCompare, int1: IntExpr },
        /// CardSet-Comparison
        CardSet{card_set: CardSet, cmp: CardSetCompare, card_set1: CardSet},
        /// StringExpr-Comparison
        String{string: StringExpr, cmp: StringCompare, string1: StringExpr},
        /// PlayerExpr-Comparison
        Player{player: PlayerExpr, cmp: PlayerCompare, player1: PlayerExpr},
        /// TeamExpr-Comparison
        Team{team: TeamExpr, cmp: TeamCompare, team1: TeamExpr},
    }

    /// Aggregating a Bool from the current game state.
    /// 
    /// # Example
    /// ```text
    /// 1 != 2
    /// "Ace" in Hand
    /// "King" not in Hand
    /// Hand empty
    /// Hand not empty
    /// current out of game successful
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregateBool {
        /// Comparisons
        Compare { cmp_bool: CompareBool},
        /// Checking if a certain String is in a CardSet
        StringInCardSet { string: StringExpr, card_set: CardSet },
        /// Checking if a certain String is not in a CardSet
        StringNotInCardSet { string: StringExpr, card_set: CardSet },
        /// Checking if CardSet is empty
        CardSetEmpty{card_set: CardSet},
        /// Checking if CardSet is not empty
        CardSetNotEmpty{card_set: CardSet},
        /// Checking if PlayerExpr or PlayerCollection is out of stage/game
        OutOfPlayer{players: Players, out_of: OutOf},
    }

    /// BoolExpr.
    /// 
    /// # Example
    /// ```text
    /// ( 1 != 2 and Hand empty )
    /// not Hand empty
    /// 1 == 2
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum BoolExpr {
        /// Binary Bool Operation
        Binary{ bool_expr: Box<BoolExpr>, op: BoolOp, bool_expr1: Box<BoolExpr>},
        /// Unary Bool Operation
        Unary { op: UnaryOp, bool_expr: Box<BoolExpr> },
        /// Aggregation Operation
        Aggregate{aggregate: AggregateBool},
    }
    // ===========================================================================

    // Team
    // ===========================================================================
    /// Aggregate a TeamExpr from the current game state.
    /// 
    /// # Example
    /// ```text
    /// team of current
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregateTeam {
        /// Team of a Player
        TeamOf { player: PlayerExpr },
    }

    /// TeamExpr.
    /// 
    /// # Example
    /// ```text
    /// T:T1
    /// team of current
    /// &T:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum TeamExpr {
        /// Name/Identifier of Team
        Literal {
            #[arbitrary(with = gen_team_name)]
            name: String
        },
        /// Aggregate Team from the current game state
        Aggregate{ aggregate: AggregateTeam},
        /// Memory
        Memory { memory: UseSingleMemory }
    }
    // ===========================================================================

    // CardPosition
    // ===========================================================================
    /// There are certain positions frequently used in a card game.
    /// For example: Draw a card refers to drawing a card from the top (most of the times).
    /// CardPosition specifies where a Card lays in a Location.
    /// 
    /// # Example
    /// ```text
    /// Hand[3]
    /// top(Hand)
    /// bottom(Hand)
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum QueryCardPosition {
        /// At a certain index of a Location
        At {
            #[arbitrary(with = gen_ident)]
            location: String, 
            int_expr:IntExpr
        },
        /// At the top of the Location
        Top{ 
            #[arbitrary(with = gen_ident)]
            location: String
        },
        /// At the bottom of the Location
        Bottom {
            #[arbitrary(with = gen_ident)]
            location: String
        },
    }

    /// Sometimes we want to get the Position of the highest or lowest Card in a Location.
    /// 
    /// # Example
    /// ```text
    /// max of Hand using ExamplePrecedence
    /// max of Hand using ExamplePointMap
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregateCardPosition {
        /// Extrema of a CardSet using a PointMap
        ExtremaPointMap { extrema: Extrema, card_set: Box<CardSet>, 
            #[arbitrary(with = gen_ident)]
            pointmap: String 
        },
        /// Extrema of a CardSet using a Precedence
        ExtremaPrecedence { extrema: Extrema, card_set: Box<CardSet>, 
            #[arbitrary(with = gen_ident)]
            precedence: String 
        },
    }

    /// CardPosition is part of the 'bigger' CardSet-logic.
    /// 
    /// # Example
    /// ```text
    /// top(Hand)
    /// max of Hand using ExamplePrecedence
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum CardPosition {
        /// Query from the CardPosition
        Query { query: QueryCardPosition },
        /// Aggregate the CardPosition from the current state
        Aggregate { aggregate: AggregateCardPosition },
    }

    // Stauts
    // ===========================================================================
    /// A Card can have different statuses. The Status of the Card is the Visibility
    /// for all Players or one Player.
    /// 
    /// # Example
    /// ```text
    /// face up
    /// face down
    /// private
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Status {
        /// Information/Card is public
        FaceUp,
        /// Information/Card is masked (nobody knows the secret)
        FaceDown,
        /// Information/Card is private (exactly one Player knows the secret)
        Private,
    }
    // ===========================================================================

    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    // Collections
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================
    /// Collection is a wrapper for all Collections. It is used for two rules (at the moment):
    /// - **Size of Collection**
    /// - **MemoryType: Collection**
    /// 
    /// # Example
    /// ```text
    /// ( 1, 2, 3, 4 )
    /// ( "Ace", "King", "Queen", "Jack" )
    /// ( Hand, Stock, Deck )
    /// ( P:Player1, current )
    /// ( T:Team1, team of current )
    /// Hand of current
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Collection {
        IntCollection { int: IntCollection },
        StringCollection { string: StringCollection },
        LocationCollection { location: LocationCollection},
        PlayerCollection {player: PlayerCollection },
        TeamCollection { team: TeamCollection },
        CardSet { card_set: Box<CardSet> },
    }

    /// IntCollection.
    /// 
    /// # Example
    /// ```text
    /// ( 1, 2, 3, 4 )
    /// &( I:ExampleIntMemory of all )
    /// &IC:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum IntCollection {
        /// A list of IntExpr
        Literal {     
            #[arbitrary(with = gen_vec_min_1_ints)]
            ints: Vec<IntExpr>
        },
        /// IntMemory of Multiple Owner (PlayerCollection/TeamCollection) aggregates to IntCollection
        AggregateMemory {
            #[arbitrary(with = gen_ident)]
            memory: String,
            multi: MultiOwner
        },
        /// Reference of a Memory that stores an IntCollection
        Memory { 
            memory: UseMemory
        },
    }

    /// StringCollection.
    /// 
    /// # Example
    /// ```text
    /// ( "Ace", "King", "Queen", "Jack" )
    /// &( S:ExampleStringMemory of all )
    /// &SC:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum StringCollection {
        /// A list of StringExpr
        Literal { 
            #[arbitrary(with = gen_vec_min_1)]
            strings: Vec<StringExpr>
        },
        /// StringMemory of Multiple Owner (PlayerCollection/TeamCollection) aggregates to StringCollection
        AggregateMemory {
            #[arbitrary(with = gen_ident)]
            memory: String,
            multi: MultiOwner
        },
        /// Reference of a Memory that stores an StringCollection
        Memory { 
            memory: UseMemory
        },
    }

    /// StringCollection.
    /// 
    /// # Example
    /// ```text
    /// ( Hand, Stock, Garbage )
    /// &LC:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum LocationCollection {
        /// A list of Location
        Literal { 
            #[arbitrary(with = gen_vec_strings)]
            locations: Vec<String>
        },
        /// Reference of a Memory that stores an LocationCollection
        Memory { 
            memory: UseMemory
        },
    }

    // PlayerCollection
    // ===========================================================================
    /// PlayerCollection that is fetched at runtime.
    /// 
    /// # Example
    /// ```text
    /// playersin
    /// playersout
    /// others
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum RuntimePlayerCollection {
        /// All players that are still in the game
        PlayersOut,
        /// All players that are not in the game
        PlayersIn,
        /// All other players that are not current
        Others,
    }

    /// PlayerCollection that aggregated from the current game state.
    /// 
    /// (Might Place Quantifier somewhere else because it does not really
    /// fit to Aggregate)
    /// 
    /// # Example
    /// ```text
    /// all
    /// any
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregatePlayerCollection {
        /// All / Any
        Quantifier { quantifier: Quantifier },
    }

    /// PlayerCollection.
    /// 
    /// (Might Place Quantifier somewhere else because it does not really
    /// fit to Aggregate)
    /// 
    /// # Example
    /// ```text
    /// ( current, next, previous )
    /// all
    /// playersin
    /// &(P:ExamplePlayerMemory of all)
    /// &PC:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum PlayerCollection {
        /// A list of PlayerExpr
        Literal { players: Vec<PlayerExpr> },
        /// Quantifier
        Aggregate { aggregate: AggregatePlayerCollection },
        Runtime { runtime: RuntimePlayerCollection },
        /// PlayerMemory of Multiple Owner (PlayerCollection/TeamCollection) aggregates to PlayerCollection
        AggregateMemory {
            memory: String, multi: MultiOwner
        },
        /// Reference of a Memory that stores an StringCollection
        Memory { memory: UseMemory },
    }
    // ===========================================================================

    // TeamCollection
    // ===========================================================================
    /// TeamCollection that is fetched at runtime.
    /// 
    /// # Example
    /// ```text
    /// other teams
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum RuntimeTeamCollection {
        /// All other Teams that are not the team of the current Player
        OtherTeams,
    }

    /// TeamCollection.
    /// 
    /// # Example
    /// ```text
    /// ( T:T1, T:T3, T:T10 )
    /// other teams
    /// &(T:ExampleTeamMemory of all)
    /// &TC:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum TeamCollection {
        /// A list of TeamExpr
        Literal { 
            teams: Vec<TeamExpr> 
        },
        Runtime {runtime: RuntimeTeamCollection },
        /// TeamMemory of Multiple Owner (PlayerCollection/TeamCollection) aggregates to TeamCollection
        AggregateMemory {
            memory: String, multi: MultiOwner
        },
        /// Reference of a Memory that stores an TeamCollection
        Memory { memory: UseMemory },
    }

    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    // CardSet
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    /// CardSet is for specifying all sets of Cards.
    /// 
    /// # Example
    /// ```text
    /// Hand
    /// Hand of current
    /// &CS:ExampleMemory
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum CardSet {
        /// Singular Group of Cards without specifying the Owner
        Group { group: Group },
        /// Group of a Cards with Owner
        GroupOwner { group: Group, owner: Owner},
        /// Reference to a Memory that stores a CardSet
        Memory { 
            memory: UseMemory
        }
    }

    /// Group combines the filter-logic with the specified Cards.
    /// 
    /// # Example
    /// ```text
    /// Hand
    /// Hand of current where same Rank
    /// Pair not in Hand
    /// Pair in Hand
    /// top(Hand)
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum Group {
        /// Location / LocaitonCollection
        Groupable { groupable: Groupable },
        /// Filtering the Groupable
        Where { groupable: Groupable, filter: FilterExpr},
        /// Combo not in Groupable
        NotCombo { combo: String, groupable: Groupable },
        /// Combo in Groupable
        Combo{ combo: String, groupable: Groupable },
        /// CardPosition
        CardPosition{ card_position: CardPosition},
    }

    // FilterExpr
    // ===========================================================================
    /// The Filter-Logic for Card-Sets.
    /// It (should) aggregates all possible Card-Sets that
    /// are queried by the filter-statements.
    /// 
    /// # Example
    /// ```text
    /// size >= 3
    /// same Rank
    /// distinct Rank
    /// adjacent Rank
    /// Rank higher than "Ace" using ExamplePrecedence
    /// Rank lower than "Ace" using ExamplePrecedence
    /// Suite is Suite of top(Hand)
    /// Suite is not Suite of top(Hand)
    /// ExampleCombo
    /// not ExampleCombo
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum AggregateFilter {
        /// Aggregate CardSets with a specific size(-range)
        Size { cmp: IntCompare, int_expr: Box<IntExpr> },
        /// Aggregate the CardSet with same Keys
        Same {
            #[arbitrary(with = gen_ident)]
            key: String
        },
        /// Aggregate the CardSet with distinct Keys
        Distinct{ 
            #[arbitrary(with = gen_ident)]
            key: String
        },
        /// Aggregate the CardSet with adjacent Keys
        Adjacent {
            #[arbitrary(with = gen_ident)]
            key: String, 
            #[arbitrary(with = gen_ident)]
            precedence: String 
        },
        /// Aggregate the CardSet with Keys higher than a specific Value
        Higher{ 
            #[arbitrary(with = gen_ident)]
            key: String,
            value: StringExpr,
            #[arbitrary(with = gen_ident)]
            precedence: String
        },
        /// Aggregate the CardSet with Keys lower than a specific Value
        Lower{
            #[arbitrary(with = gen_ident)]
            key: String, 
            value: StringExpr,
            #[arbitrary(with = gen_ident)]
            precedence: String
        },
        /// Aggregate all Cards with Key is (equal) to a certain StringExpr
        KeyIsString{ 
            #[arbitrary(with = gen_ident)]
            key: String,
            string: Box<StringExpr>
        },
        /// Aggregate all Cards with Key is not (equal) to a certain StringExpr
        KeyIsNotString{ 
            #[arbitrary(with = gen_ident)]
            key: String,
            string: Box<StringExpr>
        },
        /// Aggregate all Cards that fulfill the combo
        Combo {
            #[arbitrary(with = gen_ident)]
            combo: String
        },
        /// Aggregate all Cards that do not fulfill the combo
        NotCombo {
            #[arbitrary(with = gen_ident)]
            combo: String
        },
    }

    /// Filter Operator.
    /// 
    /// # Example
    /// ```text
    /// and
    /// or
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum FilterOp {
        /// "And" can be seen as building a cut of the two sets
        And,
        /// "Or" can be seen as building a combine of the two sets
        Or,
    }

    /// FilterExpr.
    /// 
    /// # Example
    /// ```text
    /// same Rank
    /// ( same Rank with size >= 3 )
    /// ```
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum FilterExpr {
        /// Aggregate Logic
        Aggregate {aggregate: AggregateFilter},
        /// Binary-Combination of two Filters
        Binary { filter: Box<FilterExpr>, op: FilterOp, filter1: Box<FilterExpr>},
    }
    // ===========================================================================

    // ===========================================================================
    // ===========================================================================
    // ===========================================================================

    // Game + Stage + FlowComponent + Rule
    // ===========================================================================
    // ===========================================================================
    // ===========================================================================
    /// Game is a list FlowComponents.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct Game {
        /// List of FlowComponents
        #[arbitrary(with = gen_flows_safe)]
        pub flows: Vec<FlowComponent>,
    }

    /// FlowComponent is showing the structure of each rule.
    /// Certain rules can have more Rules inside of them (all but GameRule).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum FlowComponent {
        /// Sequential Stage
        SeqStage { stage: SeqStage},
        /// Simultaneous Stage
        SimStage { stage: SimStage},
        /// Game-Rule (Terminal)
        GameRule { game_rule: GameRule},
        IfRule { if_rule: IfRule},
        ChoiceRule { choice_rule: ChoiceRule},
        OptionalRule{ optional_rule: OptionalRule},
        TriggerRule{ trigger_rule: TriggerRule},
        Conditional { conditional: Conditional},
    }

    /// SetUpRule is defining the "Game-Data" that we need to play a game.
    /// For example: We need Players, Cards, etc. to play a game.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum SetUpRule {
        /// Create a group of Players that are in the game
        CreatePlayer { 
            #[arbitrary(with = gen_vec_players_prefixed)]
            players: Vec<String> 
        },
        /// Create a group of Teams that are in the game
        CreateTeams {
            #[arbitrary(with = gen_vec_teams_with_players)]
            teams: Vec<(String, PlayerCollection)>
        },
        /// Create a Turn-Order the Players follow
        CreateTurnorder {player_collection: PlayerCollection},
        /// Create a random Turn-Order the Players follow
        CreateTurnorderRandom { player_collection: PlayerCollection},
        /// Create Locations on a specific Owner.
        CreateLocation { 
            #[arbitrary(with = gen_vec_strings)]
            locations: Vec<String>, 
            owner: Owner 
        },
        /// Create Cards on a Location
        CreateCardOnLocation { 
            #[arbitrary(with = gen_ident)]
            location: String, 
            #[arbitrary(with = gen_vec_min_1)]
            cards: Vec<Types>
        },
        /// Create Tokens on a Location
        CreateTokenOnLocation { int: IntExpr, 
            #[arbitrary(with = gen_ident)]
            token: String,
            #[arbitrary(with = gen_ident)]
            location: String 
        },
        /// Create Combo with a Filter (for later use)
        CreateCombo {
            #[arbitrary(with = gen_ident)]
            combo: String, 
            filter: FilterExpr
        },
        /// Create a Memory with a Memory-Type (for later use)
        CreateMemoryWithMemoryType {
            #[arbitrary(with = gen_ident)]
            memory: String,
            memory_type: MemoryType, 
            owner: Owner
        },
        /// Create a Memory without a Memory-Type (for later use)
        CreateMemory { 
            #[arbitrary(with = gen_ident)]
            memory: String,
            owner: Owner 
        },
        /// Create a Precedence on Key-Value-Pairs
        CreatePrecedence {
            #[arbitrary(with = gen_ident)]
            precedence: String, 
            #[arbitrary(with = gen_vec_min_1_kvs)]
            kvs: Vec<(String, String)>
        },
        /// Create a PointMap on Key-Value-Pairs
        CreatePointMap { 
            #[arbitrary(with = gen_ident)]
            pointmap: String, 
            #[arbitrary(with = gen_vec_min_1_kvis)]
            kvis: Vec<(String, String, IntExpr)> 
        },
    }


    /// Action-Rule defines permutations of the game state that
    /// are not creations of new things.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum ActionRule {
        /// Give a CardSet a different Status
        FlipAction { card_set: CardSet, status: Status },
        /// Shuffle a CardSet
        ShuffleAction {card_set: CardSet},
        /// Set a Player/PlayerCollection out of stage, game, etc.
        OutAction {players: Players, out_of: OutOf},
        /// Assign a Value to a Memory
        SetMemory{
            #[arbitrary(with = gen_ident)]
            memory: String,
            memory_type: MemoryType
        },
        /// Reset Memory (first initialization?)
        ResetMemory {
            #[arbitrary(with = gen_ident)]
            memory: String
        },
        /// Cycle to a certain Player (End turn of the current Player and set other player active)
        CycleAction {player:PlayerExpr},
        /// Bid a certain amount
        BidAction{quantitiy: Quantity},
        /// Bid a certain amount and store it in a Memory
        BidMemoryAction{ 
            #[arbitrary(with = gen_ident)]
            memory: String,
            quantity: Quantity,
            owner: Owner,
        },
        /// End Turn, End Stage, etc.
        EndAction{end_type: EndType},
        /// Demand certain information
        DemandAction{demand_type: DemandType},
        /// Demand certain information and store it in Memory
        DemandMemoryAction{demand_type: DemandType, 
            #[arbitrary(with = gen_ident)]
            memory: String
        },
        /// Move CardSets around
        Move{move_type: MoveType},
    }

    /// Evaluate a Score or a Winner.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum ScoringRule {
        /// Permutate the Score
        ScoreRule{score_rule: ScoreRule},
        /// Evaluate a Winner
        WinnerRule{ winner_rule: WinnerRule},
    }

    /// Terminal Actions:
    /// - **SetUpRule**
    /// - **ActionRule**
    /// - **ScoringRule**
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum GameRule {
        SetUp { setup: SetUpRule},
        Action{ action: ActionRule},
        Scoring{scoring: ScoringRule},
    }

    /// Sequential Stage can be seen as a 'while-loop'.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct SeqStage {
        /// Name/Identifier
        #[arbitrary(with = gen_ident)]
        pub stage: String,
        /// For a certain Player
        pub player: PlayerExpr,
        pub end_condition: EndCondition,
        #[arbitrary(with = gen_flows_safe)]
        pub flows: Vec<FlowComponent>,
    }

    /// Simultaneous Stage can be seen as a complex 'while-loop' with multiple Players all at once.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct SimStage {
        /// Name/Identifier
        #[arbitrary(with = gen_ident)]
        pub stage: String,
        /// For a certain gtoup of Players
        pub players: PlayerCollection,
        pub end_condition: EndCondition,
        #[arbitrary(with = gen_flows_safe)]
        pub flows: Vec<FlowComponent>,
    }

    /// Case can be seen as an If-Rule but with the option of no Condition.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum Case {
        NoBool{
            #[arbitrary(with = gen_flows_safe)]
            flows: Vec<FlowComponent>
        },
        Bool{
            bool_expr: BoolExpr, 
            #[arbitrary(with = gen_flows_safe)]
            flows: Vec<FlowComponent>
        },
    }

    /// Conditional is either a If-Statement after If-Statement or more likely an If-Else.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct Conditional {
        #[arbitrary(with = gen_vec_min_1)]
        pub cases: Vec<Case>,
    }

    /// If-Rule is a basic If-Statement.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct IfRule {
        pub condition: BoolExpr,
        #[arbitrary(with = gen_flows_safe)]
        pub flows: Vec<FlowComponent>,
    }

    /// Optional-Rule is a Rule that can be executed optionally.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct OptionalRule {
        #[arbitrary(with = gen_flows_safe)]
        pub flows: Vec<FlowComponent>,
    }

    /// Choice-Rule is a Rule where you have to choose an option from a set of options.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct ChoiceRule {
        #[arbitrary(with = gen_flows_safe)]
        pub options: Vec<FlowComponent>,
    }

    /// Used in Simultaneous Stage. 
    /// If you are the first one that reaches it you can do the flows inside (the others can not anymore).
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub struct TriggerRule {
        #[arbitrary(with = gen_flows_safe)]
        pub flows: Vec<FlowComponent>,
    }

    /// Different ways of moving set of cards around.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum MoveType {
        Deal { deal: DealMove },
        Exchange { exchange: ExchangeMove},
        Classic{ classic: ClassicMove},
        Place{ token: TokenMove},
    }

    /// Move a CardSet to another CardSet.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum MoveCardSet {
        Move { from: CardSet, status: Status, to: CardSet },
        MoveQuantity { quantity: Quantity, from: CardSet, status: Status, to: CardSet},
    }

    /// Move a CardSet to another CardSet
    /// by simply removing the first one and the adding it to the second one.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum ClassicMove {
        MoveCardSet {move_cs: MoveCardSet},
    }

    /// Deal is basically a classic-move?
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum DealMove {
        MoveCardSet {deal_cs: MoveCardSet},
    }

    /// Exchange CardSets
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum ExchangeMove {
        MoveCardSet {exchange_cs: MoveCardSet},
    }

    /// Place tokens on a Location.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum TokenMove {
        Place {
            #[arbitrary(with = gen_ident)]
            token: String,
            from_loc: TokenLocExpr, to_loc: TokenLocExpr},
        PlaceQuantity {quantity: Quantity, 
            #[arbitrary(with = gen_ident)]
            token: String,
            from_loc: TokenLocExpr, to_loc: TokenLocExpr},
    }

    /// Define a Token-Location for Tokens.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum TokenLocExpr {
        Groupable{ groupable: Groupable},
        GroupablePlayers { groupable: Groupable, players: Players},
    }

    /// Score Points to a Player or to a Memory.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum ScoreRule {
        Score {int: IntExpr, players: Players},
        ScoreMemory {int: IntExpr, 
            #[arbitrary(with = gen_ident)]
            memory: String,
            players: Players
        },
    }

    /// There are specific types in a game that can determine a win like a score of a Player.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum WinnerType {
        Score,
        Memory{ 
            #[arbitrary(with = gen_ident)]
            memory: String
        },
        Position,
    }

    /// Declares the Winner/Winners of the game. 
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Arbitrary)]
    pub enum WinnerRule {
        Winner {players: Players},
        WinnerWith {extrema: Extrema, winner_type: WinnerType},
    }

    // ===========================================================================
    // ===========================================================================
    // ===========================================================================
}