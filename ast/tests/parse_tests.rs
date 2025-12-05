#[cfg(test)]
mod tests {
    use super::*;
    use ast::ast::*;
    use syn::parse_str;

    // PlayerExpr ============================================================
    #[test]
    fn parses_valid_player_current() {
        let parsed: PlayerExpr = parse_str(
          "current"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::Current);
    }

    #[test]
    fn parses_valid_player_previous() {
        let parsed: PlayerExpr = parse_str(
          "previous"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::Previous);
    }

    #[test]
    fn parses_valid_player_competitor() {
        let parsed: PlayerExpr = parse_str(
          "competitor"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::Competitor);
    }

    #[test]
    fn parses_valid_player_owner_highest() {
        let parsed: PlayerExpr = parse_str(
          "owner of highest P1"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::OwnerOfHighest("P1".to_string()));
    }

    #[test]
    fn parses_valid_player_owner_lowest() {
        let parsed: PlayerExpr = parse_str(
          "owner of lowest P1"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::OwnerOfLowest("P1".to_string()));
    }

    #[test]
    fn parses_valid_player_turnorder() {
        let parsed: PlayerExpr = parse_str(
          "turnorder(3)"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::Turnorder(IntExpr::Int(3)));
    }
    
    #[test]
    fn parses_valid_player_id() {
        let parsed: PlayerExpr = parse_str(
          "P1"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::PlayerName("P1".to_string()));
    }

    // =======================================================================

    // Op ====================================================================
    #[test]
    fn parses_valid_op_plus() {
        let parsed: Op = parse_str(
          "+"
        ).unwrap();
        assert_eq!(parsed, Op::Plus);
    }

    #[test]
    fn parses_valid_op_minus() {
        let parsed: Op = parse_str(
          "-"
        ).unwrap();
        assert_eq!(parsed, Op::Minus);
    }

    #[test]
    fn parses_valid_op_div() {
        let parsed: Op = parse_str(
          "/"
        ).unwrap();
        assert_eq!(parsed, Op::Div);
    }

    #[test]
    fn parses_valid_op_mul() {
        let parsed: Op = parse_str(
          "*"
        ).unwrap();
        assert_eq!(parsed, Op::Mul);
    }
    
    #[test]
    fn parses_valid_op_mod() {
        let parsed: Op = parse_str(
          "%"
        ).unwrap();
        assert_eq!(parsed, Op::Mod);
    }
    // =======================================================================

    // IntCmpOp ==============================================================
    #[test]
    fn parses_valid_intcmpop_eq() {
        let parsed: IntCmpOp = parse_str(
          "=="
        ).unwrap();
        assert_eq!(parsed, IntCmpOp::Eq);
    }

    #[test]
    fn parses_valid_intcmpop_neq() {
        let parsed: IntCmpOp = parse_str(
          "!="
        ).unwrap();
        assert_eq!(parsed, IntCmpOp::Neq);
    }

    #[test]
    fn parses_valid_intcmpop_le() {
        let parsed: IntCmpOp = parse_str(
          "<="
        ).unwrap();
        assert_eq!(parsed, IntCmpOp::Le);
    }

    #[test]
    fn parses_valid_intcmpop_ge() {
        let parsed: IntCmpOp = parse_str(
          ">="
        ).unwrap();
        assert_eq!(parsed, IntCmpOp::Ge);
    }

    #[test]
    fn parses_valid_intcmpop_lt() {
        let parsed: IntCmpOp = parse_str(
          "<"
        ).unwrap();
        assert_eq!(parsed, IntCmpOp::Lt);
    }

    #[test]
    fn parses_valid_intcmpop_gt() {
        let parsed: IntCmpOp = parse_str(
          ">"
        ).unwrap();
        assert_eq!(parsed, IntCmpOp::Gt);
    }
    
    // =======================================================================

    // Status ================================================================

    #[test]
    fn parses_valid_status_facup() {
        let parsed: Status = parse_str(
          "face up"
        ).unwrap();
        assert_eq!(parsed, Status::FaceUp);
    }

    #[test]
    fn parses_valid_facedown() {
        let parsed: Status = parse_str(
          "face down"
        ).unwrap();
        assert_eq!(parsed, Status::FaceDown);
    }
    
    #[test]
    fn parses_valid_private() {
        let parsed: Status = parse_str(
          "private"
        ).unwrap();
        assert_eq!(parsed, Status::Private);
    }
    
    // =======================================================================

    // Quantifier ============================================================
    #[test]
    fn parses_valid_quantifier_all() {
        let parsed: Quantifier = parse_str(
          "all"
        ).unwrap();
        assert_eq!(parsed, Quantifier::All);
    }

    #[test]
    fn parses_valid_quantifier_any() {
        let parsed: Quantifier = parse_str(
          "any"
        ).unwrap();
        assert_eq!(parsed, Quantifier::Any);
    }

    // =======================================================================

    // TeamExpr ==============================================================
    #[test]
    fn parses_valid_teamexpr_team_of() {
        let parsed: TeamExpr = parse_str(
          "team of current"
        ).unwrap();
        assert_eq!(parsed, TeamExpr::TeamOf(PlayerExpr::Current));
    }

    #[test]
    fn parses_valid_teamexpr_team_id() {
        let parsed: TeamExpr = parse_str(
          "T1"
        ).unwrap();
        assert_eq!(parsed, TeamExpr::TeamName("T1".to_string()));
    }

    // =======================================================================

    // LocationExpr ==========================================================
    #[test]
    fn parses_valid_locationexpr_of_team() {
        let parsed: LocationExpr = parse_str(
          "hand of team(team of current)"
        ).unwrap();
        assert_eq!(parsed, LocationExpr::LocationTeam("hand".to_string(), TeamExpr::TeamOf(PlayerExpr::Current)));
    }

    #[test]
    fn parses_valid_locationexpr_of_player() {
        let parsed: LocationExpr = parse_str(
          "hand of player(current)"
        ).unwrap();
        assert_eq!(parsed, LocationExpr::LocationPlayer("hand".to_string(), PlayerExpr::Current));
    }

    #[test]
    fn parses_valid_locationexpr_of_current_or_table() {
        let parsed: LocationExpr = parse_str(
          "hand"
        ).unwrap();
        assert_eq!(parsed, LocationExpr::LocationCurrentOrTable("hand".to_string()));
    }

    // =======================================================================

    // CardPosition ==========================================================

    #[test]
    fn parses_valid_cardposition_top_of() {
        let parsed: CardPosition = parse_str(
          "top of hand"
        ).unwrap();
        assert_eq!(parsed, CardPosition::Top(LocationExpr::LocationCurrentOrTable("hand".to_string())));
    }

    #[test]
    fn parses_valid_cardposition_bottom_of() {
        let parsed: CardPosition = parse_str(
          "bottom of hand"
        ).unwrap();
        assert_eq!(parsed, CardPosition::Bottom(LocationExpr::LocationCurrentOrTable("hand".to_string())));
    }

    #[test]
    fn parses_valid_cardposition_max_of_using_prec() {
        let parsed: CardPosition = parse_str(
          "max of hand using prec(aces)"
        ).unwrap();
        assert_eq!(parsed, CardPosition::MaxPrec(Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string()));
    }

    #[test]
    fn parses_valid_cardposition_min_of_using_prec() {
        let parsed: CardPosition = parse_str(
          "min of hand using prec(aces)"
        ).unwrap();
        assert_eq!(parsed, CardPosition::MinPrec(Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string()));
    }

    #[test]
    fn parses_valid_cardposition_max_of_using_point() {
        let parsed: CardPosition = parse_str(
          "max of hand using point(aces)"
        ).unwrap();
        assert_eq!(parsed, CardPosition::MaxPoint(Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string()));
    }

    #[test]
    fn parses_valid_cardposition_min_of_using_point() {
        let parsed: CardPosition = parse_str(
          "min of hand using point(aces)"
        ).unwrap();
        assert_eq!(parsed, CardPosition::MinPoint(Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string()));
    }

    #[test]
    fn parses_valid_cardposition_at() {
        let parsed: CardPosition = parse_str(
          "hand[3]"
        ).unwrap();
        assert_eq!(parsed, CardPosition::At(LocationExpr::LocationCurrentOrTable("hand".to_string()), IntExpr::Int(3)));
    }

    // =======================================================================

    // IntExpr ===============================================================

    #[test]
    fn parses_valid_intexpr_int() {
        let parsed: IntExpr = parse_str(
          "3"
        ).unwrap();
        assert_eq!(parsed, IntExpr::Int(3));
    }

    #[test]
    fn parses_valid_intexpr_op() {
        let parsed: IntExpr = parse_str(
          "(3 + 3)"
        ).unwrap();
        assert_eq!(parsed, IntExpr::IntOp(Box::new(IntExpr::Int(3)), Op::Plus, Box::new(IntExpr::Int(3))));
    }

    #[test]
    fn parses_valid_intexpr_size_of() {
        let parsed: IntExpr = parse_str(
          "size of (3, 3)"
        ).unwrap();
        assert_eq!(parsed, IntExpr::SizeOf(Collection::IntCollection(
          IntCollection {
            ints: vec![IntExpr::Int(3), IntExpr::Int(3)]
          }
        )));
    }

    #[test]
    fn parses_valid_intexpr_sum() {
        let parsed: IntExpr = parse_str(
          "sum(3, 3)"
        ).unwrap();
        assert_eq!(parsed, IntExpr::SumOfIntCollection(
          IntCollection {
            ints: vec![IntExpr::Int(3), IntExpr::Int(3)]
          }
        ));
    }
    
    #[test]
    fn parses_valid_intexpr_sum_of() {
        let parsed: IntExpr = parse_str(
          "sum of hand using aces"
        ).unwrap();
        assert_eq!(parsed, IntExpr::SumOfCardSet(
          Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string())
        );
    }

    #[test]
    fn parses_valid_intexpr_min_intcollection() {
        let parsed: IntExpr = parse_str(
          "min(3, 3)"
        ).unwrap();
        assert_eq!(parsed, IntExpr::MinIntCollection(
          IntCollection {
            ints: vec![IntExpr::Int(3), IntExpr::Int(3)]
          }
        ));
    }
    
    #[test]
    fn parses_valid_intexpr_max_intcollection() {
        let parsed: IntExpr = parse_str(
          "max(3, 3)"
        ).unwrap();
        assert_eq!(parsed, IntExpr::MaxIntCollection(
          IntCollection {
            ints: vec![IntExpr::Int(3), IntExpr::Int(3)]
          }
        ));
    }
    
    #[test]
    fn parses_valid_intexpr_min_pointmap() {
        let parsed: IntExpr = parse_str(
          "min of hand using aces"
        ).unwrap();
        assert_eq!(parsed, IntExpr::MinOf(
          Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string())
        );
    }
    
    #[test]
    fn parses_valid_intexpr_max_pointmap() {
        let parsed: IntExpr = parse_str(
          "max of hand using aces"
        ).unwrap();
        assert_eq!(parsed, IntExpr::MaxOf(
          Box::new(CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))), "aces".to_string())
        );
    }
    
    #[test]
    fn parses_valid_intexpr_stageroundcounter() {
        let parsed: IntExpr = parse_str(
          "stageroundcounter"
        ).unwrap();
        assert_eq!(parsed, IntExpr::StageRoundCounter);
    }

    // =======================================================================

    // BoolExpr ==============================================================

    #[test]
    fn parses_valid_boolexpr_stringeq() {
        let parsed: BoolExpr = parse_str(
          "A == B"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::StringEq(StringExpr::ID("A".to_string()), StringExpr::ID("B".to_string())));
    }

    #[test]
    fn parses_valid_boolexpr_stringneq() {
        let parsed: BoolExpr = parse_str(
          "A != B"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::StringNeq(StringExpr::ID("A".to_string()), StringExpr::ID("B".to_string())));
    }

    #[test]
    fn parses_valid_boolexpr_playereq() {
        let parsed: BoolExpr = parse_str(
          "player(A == B)"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::PlayerEq(PlayerExpr::PlayerName("A".to_string()), PlayerExpr::PlayerName("B".to_string())));
    }

    #[test]
    fn parses_valid_boolexpr_playerneq() {
        let parsed: BoolExpr = parse_str(
          "player(A != B)"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::PlayerNeq(PlayerExpr::PlayerName("A".to_string()), PlayerExpr::PlayerName("B".to_string())));
    }
    
    #[test]
    fn parses_valid_boolexpr_team_eq() {
        let parsed: BoolExpr = parse_str(
          "team(A == B)"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::TeamEq(TeamExpr::TeamName("A".to_string()), TeamExpr::TeamName("B".to_string())));
    }

    #[test]
    fn parses_valid_boolexpr_team_neq() {
        let parsed: BoolExpr = parse_str(
          "team(A != B)"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::TeamNeq(TeamExpr::TeamName("A".to_string()), TeamExpr::TeamName("B".to_string())));
    }

    #[test]
    fn parses_valid_boolexpr_or() {
        let parsed: BoolExpr = parse_str(
          "(player(A != B) or player(A != B))"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::Or(
          Box::new(BoolExpr::PlayerNeq(PlayerExpr::PlayerName("A".to_string()), PlayerExpr::PlayerName("B".to_string()))),
          Box::new(BoolExpr::PlayerNeq(PlayerExpr::PlayerName("A".to_string()), PlayerExpr::PlayerName("B".to_string())))
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_and() {
        let parsed: BoolExpr = parse_str(
          "(player(A != B) and player(A != B))"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::And(
          Box::new(BoolExpr::PlayerNeq(PlayerExpr::PlayerName("A".to_string()), PlayerExpr::PlayerName("B".to_string()))),
          Box::new(BoolExpr::PlayerNeq(PlayerExpr::PlayerName("A".to_string()), PlayerExpr::PlayerName("B".to_string())))
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_intcmp() {
        let parsed: BoolExpr = parse_str(
          "3 == 2"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::IntCmp(
          IntExpr::Int(3),
          IntCmpOp::Eq,
          IntExpr::Int(2)
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_cardset_eq() {
        let parsed: BoolExpr = parse_str(
          "cards(hand == hand)"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::CardSetEq(
          CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string()))),
          CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string()))),
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_cardset_neq() {
        let parsed: BoolExpr = parse_str(
          "cards(hand != hand)"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::CardSetNeq(
          CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string()))),
          CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string()))),
        ));
    }

    #[test]
    fn parses_valid_boolexpr_cardset_empty() {
        let parsed: BoolExpr = parse_str(
          "hand is empty"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::CardSetIsEmpty(
          CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))
        ));
    }

    #[test]
    fn parses_valid_boolexpr_cardset_not_empty() {
        let parsed: BoolExpr = parse_str(
          "hand is not empty"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::CardSetIsNotEmpty(
          CardSet::Group(Group::Location(LocationExpr::LocationCurrentOrTable("hand".to_string())))
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_not() {
        let parsed: BoolExpr = parse_str(
          "not 3 == 2"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::Not(
          Box::new(BoolExpr::IntCmp(
            IntExpr::Int(3),
            IntCmpOp::Eq,
            IntExpr::Int(2)
        ))));
    }
    
    #[test]
    fn parses_valid_boolexpr_out_of_stage_player() {
        let parsed: BoolExpr = parse_str(
          "current out of stage"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::OutOfStagePlayer(
          PlayerExpr::Current
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_out_of_game_player() {
        let parsed: BoolExpr = parse_str(
          "current out of game"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::OutOfGamePlayer(
          PlayerExpr::Current
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_out_of_stage_collection() {
        let parsed: BoolExpr = parse_str(
          "others out of stage"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::OutOfStageCollection(
          PlayerCollection::Others
        ));
    }
    
    #[test]
    fn parses_valid_boolexpr_out_of_game_collection() {
        let parsed: BoolExpr = parse_str(
          "others out of game"
        ).unwrap();
        assert_eq!(parsed, BoolExpr::OutOfGameCollection(
          PlayerCollection::Others
        ));
    }
    
    // =======================================================================

    // StringExpr ============================================================
    
    #[test]
    fn parses_valid_stringexpr_id() {
        let parsed: StringExpr = parse_str(
          "Monkey"
        ).unwrap();
        assert_eq!(parsed, StringExpr::ID(
          "Monkey".to_string()
        ));
    }

    #[test]
    fn parses_valid_stringexpr_key_of() {
        let parsed: StringExpr = parse_str(
          "rank of top of hand"
        ).unwrap();
        assert_eq!(parsed, StringExpr::KeyOf(
          "rank".to_string(),
          CardPosition::Top(LocationExpr::LocationCurrentOrTable("hand".to_string()))
        ));
    }

    #[test]
    fn parses_valid_stringexpr_collection_at() {
        let parsed: StringExpr = parse_str(
          "(A, B, C)[3]"
        ).unwrap();
        assert_eq!(parsed, StringExpr::StringCollectionAt(
          StringCollection {
            strings: vec![
              StringExpr::ID("A".to_string()),
              StringExpr::ID("B".to_string()),
              StringExpr::ID("C".to_string())
            ]
          },
          IntExpr::Int(3)
        ));
    }

    // =======================================================================

    // PlayerCollection ======================================================
   
    #[test]
    fn parses_valid_player_collection_others() {
        let parsed: PlayerCollection = parse_str(
          "others"
        ).unwrap();
        assert_eq!(parsed, 
          PlayerCollection::Others
        );
    }

    #[test]
    fn parses_valid_player_collection_playersin() {
        let parsed: PlayerCollection = parse_str(
          "playersin"
        ).unwrap();
        assert_eq!(parsed, 
          PlayerCollection::PlayersIn
        );
    }

    #[test]
    fn parses_valid_player_collection_playersout() {
        let parsed: PlayerCollection = parse_str(
          "playersout"
        ).unwrap();
        assert_eq!(parsed, 
          PlayerCollection::PlayersOut
        );
    }

    #[test]
    fn parses_valid_player_collection_collection() {
        let parsed: PlayerCollection = parse_str(
          "(current, current)"
        ).unwrap();
        assert_eq!(parsed, 
          PlayerCollection::Player(
            vec![
              PlayerExpr::Current,
              PlayerExpr::Current,
            ]
          )
        );
    }

    #[test]
    fn parses_valid_player_collection_quantifier() {
        let parsed: PlayerCollection = parse_str(
          "all"
        ).unwrap();
        assert_eq!(parsed, 
          PlayerCollection::Quantifier(
            Quantifier::All
          )
        );
    }

    // =======================================================================

    // FilterExpr ============================================================

    #[test]
    fn parses_valid_filter_expr_same_key() {
        let parsed: FilterExpr = parse_str(
          "same rank"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Same("rank".to_string())
        );
    }

    #[test]
    fn parses_valid_filter_expr_distinct_key() {
        let parsed: FilterExpr = parse_str(
          "distinct rank"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Distinct("rank".to_string())
        );
    }

    #[test]
    fn parses_valid_filter_expr_adjacent_key() {
        let parsed: FilterExpr = parse_str(
          "adjacent rank using aces"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Adjacent("rank".to_string(), "aces".to_string())
        );
    }

    #[test]
    fn parses_valid_filter_expr_higher_key() {
        let parsed: FilterExpr = parse_str(
          "higher rank using aces"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Higher("rank".to_string(), "aces".to_string())
        );
    }

    #[test]
    fn parses_valid_filter_expr_lower_key() {
        let parsed: FilterExpr = parse_str(
          "lower rank using aces"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Lower("rank".to_string(), "aces".to_string())
        );
    }

    #[test]
    fn parses_valid_filter_expr_size_eq() {
        let parsed: FilterExpr = parse_str(
          "size == 3"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::SizeEq(Box::new(IntExpr::Int(3)))
        );
    }

    #[test]
    fn parses_valid_filter_expr_size_neq() {
        let parsed: FilterExpr = parse_str(
          "size != 3"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::SizeNeq(Box::new(IntExpr::Int(3)))
        );
    }

    #[test]
    fn parses_valid_filter_expr_size_lt() {
        let parsed: FilterExpr = parse_str(
          "size < 3"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::SizeLt(Box::new(IntExpr::Int(3)))
        );
    }

    #[test]
    fn parses_valid_filter_expr_size_gt() {
        let parsed: FilterExpr = parse_str(
          "size > 3"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::SizeGt(Box::new(IntExpr::Int(3)))
        );
    }

    #[test]
    fn parses_valid_filter_expr_size_le() {
        let parsed: FilterExpr = parse_str(
          "size <= 3"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::SizeLe(Box::new(IntExpr::Int(3)))
        );
    }

    #[test]
    fn parses_valid_filter_expr_size_ge() {
        let parsed: FilterExpr = parse_str(
          "size >= 3"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::SizeGe(Box::new(IntExpr::Int(3)))
        );
    }

    #[test]
    fn parses_valid_filter_expr_rank_eq() {
        let parsed: FilterExpr = parse_str(
          "Key(rank == ace)"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::KeyEq(
            "rank".to_string(),
            Box::new(StringExpr::ID("ace".to_string())))
        );
    }

    #[test]
    fn parses_valid_filter_expr_rank_neq() {
        let parsed: FilterExpr = parse_str(
          "Key(rank != ace)"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::KeyNeq(
            "rank".to_string(),
            Box::new(StringExpr::ID("ace".to_string())))
        );
    }

    #[test]
    fn parses_valid_filter_expr_not_combo() {
        let parsed: FilterExpr = parse_str(
          "not Pair"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::NotCombo(
            "Pair".to_string()
          )
        );
    }

    #[test]
    fn parses_valid_filter_expr_combo() {
        let parsed: FilterExpr = parse_str(
          "Pair"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Combo(
            "Pair".to_string()
          )
        );
    }

    #[test]
    fn parses_valid_filter_expr_and() {
        let parsed: FilterExpr = parse_str(
          "(Pair and Triple)"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::And(
            Box::new(FilterExpr::Combo(
              "Pair".to_string()
            )),
            Box::new(FilterExpr::Combo(
              "Triple".to_string()
            ))
          )
        );
    }

    #[test]
    fn parses_valid_filter_expr_or() {
        let parsed: FilterExpr = parse_str(
          "(Pair or Triple)"
        ).unwrap();
        assert_eq!(parsed, 
          FilterExpr::Or(
            Box::new(FilterExpr::Combo(
              "Pair".to_string()
            )),
            Box::new(FilterExpr::Combo(
              "Triple".to_string()
            ))
          )
        );
    }

    // =======================================================================

    // Group =================================================================

    #[test]
    fn parses_valid_group_location() {
        let parsed: Group = parse_str(
          "hand"
        ).unwrap();
        assert_eq!(parsed, 
          Group::Location(
            LocationExpr::LocationCurrentOrTable("hand".to_string())
          )
        );
    }

    #[test]
    fn parses_valid_group_location_filter() {
        let parsed: Group = parse_str(
          "hand where same rank"
        ).unwrap();
        assert_eq!(parsed, 
          Group::LocationWhere(
            LocationExpr::LocationCurrentOrTable("hand".to_string()),
            FilterExpr::Same("rank".to_string())
          )
        );
    }

    #[test]
    fn parses_valid_group_location_collection() {
        let parsed: Group = parse_str(
          "(hand, stack)"
        ).unwrap();
        assert_eq!(parsed, 
          Group::LocationCollection(
            LocationCollection {
              locations: vec![
                LocationExpr::LocationCurrentOrTable("hand".to_string()),
                LocationExpr::LocationCurrentOrTable("stack".to_string())
              ]
            }
          )
        );
    }

    #[test]
    fn parses_valid_group_location_collection_filter() {
        let parsed: Group = parse_str(
          "(hand, stack) where same rank"
        ).unwrap();
        assert_eq!(parsed, 
          Group::LocationCollectionWhere(
            LocationCollection {
              locations: vec![
                LocationExpr::LocationCurrentOrTable("hand".to_string()),
                LocationExpr::LocationCurrentOrTable("stack".to_string())
              ]
            },
            FilterExpr::Same("rank".to_string())
          )
        );
    }


    #[test]
    fn parses_valid_group_combo_in_location() {
        let parsed: Group = parse_str(
          "Pair in hand"
        ).unwrap();
        assert_eq!(parsed, 
          Group::ComboInLocation(
            "Pair".to_string(),
            LocationExpr::LocationCurrentOrTable("hand".to_string())
          )
        );
    }

    #[test]
    fn parses_valid_group_combo_in_location_collection() {
        let parsed: Group = parse_str(
          "Pair in (hand, stack)"
        ).unwrap();
        assert_eq!(parsed, 
          Group::ComboInLocationCollection(
            "Pair".to_string(),
            LocationCollection {
              locations: vec![
                LocationExpr::LocationCurrentOrTable("hand".to_string()),
                LocationExpr::LocationCurrentOrTable("stack".to_string())
              ]
            }
          )
        );
    }

    #[test]
    fn parses_valid_group_combo_not_in_location() {
        let parsed: Group = parse_str(
          "Pair not in hand"
        ).unwrap();
        assert_eq!(parsed, 
          Group::NotComboInLocation(
            "Pair".to_string(),
            LocationExpr::LocationCurrentOrTable("hand".to_string())
          )
        );
    }

    #[test]
    fn parses_valid_group_combo_not_in_location_collection() {
        let parsed: Group = parse_str(
          "Pair not in (hand, stack)"
        ).unwrap();
        assert_eq!(parsed, 
          Group::NotComboInLocationCollection(
            "Pair".to_string(),
            LocationCollection {
              locations: vec![
                LocationExpr::LocationCurrentOrTable("hand".to_string()),
                LocationExpr::LocationCurrentOrTable("stack".to_string())
              ]
            }
          )
        );
    }

    #[test]
    fn parses_valid_group_cardposition() {
        let parsed: Group = parse_str(
          "top of hand"
        ).unwrap();
        assert_eq!(parsed, 
          Group::CardPosition(
            CardPosition::Top(LocationExpr::LocationCurrentOrTable("hand".to_string()))
          )
        );
    }

    // =======================================================================

    // CardSet ===============================================================

    #[test]
    fn parses_valid_cardset_group() {
        let parsed: CardSet = parse_str(
          "top of hand"
        ).unwrap();
        assert_eq!(parsed, 
          CardSet::Group(
            Group::CardPosition(
              CardPosition::Top(LocationExpr::LocationCurrentOrTable("hand".to_string()))
            )
          )
        );
    }

    #[test]
    fn parses_valid_cardset_group_of_player() {
        let parsed: CardSet = parse_str(
          "hand where same rank of current"
        ).unwrap();
        assert_eq!(parsed, 
          CardSet::GroupOfPlayer(
            Group::LocationWhere(
              LocationExpr::LocationCurrentOrTable("hand".to_string()),
              FilterExpr::Same("rank".to_string())
            ),
            PlayerExpr::Current
          )
        );
    }

    #[test]
    fn parses_valid_cardset_group_of_player_collection() {
        let parsed: CardSet = parse_str(
          "hand where same rank of others"
        ).unwrap();
        assert_eq!(parsed, 
          CardSet::GroupOfPlayerCollection(
            Group::LocationWhere(
              LocationExpr::LocationCurrentOrTable("hand".to_string()),
              FilterExpr::Same("rank".to_string())
            ),
            PlayerCollection::Others
          )
        );
    }
}