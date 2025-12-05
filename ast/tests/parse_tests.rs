#[cfg(test)]
mod tests {
    use super::*;
    use ast::ast::{PlayerCollection, PlayerExpr, IntExpr};
    use syn::parse_str;

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
          "owner of highest flsj"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::OwnerOfHighest("flsj".to_string()));
    }

    #[test]
    fn parses_valid_player_owner_lowest() {
        let parsed: PlayerExpr = parse_str(
          "owner of lowest flsj"
        ).unwrap();
        assert_eq!(parsed, PlayerExpr::OwnerOfLowest("flsj".to_string()));
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
}