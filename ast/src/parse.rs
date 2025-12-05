use crate::ast::*;

use bincode::Error;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, LitInt, Token, braced, bracketed, parenthesized};


// ------------------------
// Keywords declarations
// ------------------------

mod kw {
  syn::custom_keyword!(cards);
  syn::custom_keyword!(face);
  syn::custom_keyword!(down);
  syn::custom_keyword!(up);
  syn::custom_keyword!(private);
  syn::custom_keyword!(all);
  syn::custom_keyword!(any);

  syn::custom_keyword!(current);
  syn::custom_keyword!(previous);
  syn::custom_keyword!(owner);
  syn::custom_keyword!(of);
  syn::custom_keyword!(highest);
  syn::custom_keyword!(lowest);
  syn::custom_keyword!(competitor);
  syn::custom_keyword!(turnorder);
  syn::custom_keyword!(top);
  syn::custom_keyword!(bottom);
  syn::custom_keyword!(team);
  syn::custom_keyword!(at);
  syn::custom_keyword!(using);
  syn::custom_keyword!(prec);
  syn::custom_keyword!(point);
  syn::custom_keyword!(min);
  syn::custom_keyword!(max);
  syn::custom_keyword!(stageroundcounter);
  syn::custom_keyword!(size);
  syn::custom_keyword!(sum);
  syn::custom_keyword!(or);
  syn::custom_keyword!(and);
  syn::custom_keyword!(stage);
  syn::custom_keyword!(game);
  syn::custom_keyword!(not);
  syn::custom_keyword!(is);
  syn::custom_keyword!(empty);
  syn::custom_keyword!(out);
  syn::custom_keyword!(players);
  syn::custom_keyword!(playersin);
  syn::custom_keyword!(playersout);
  syn::custom_keyword!(others);
  syn::custom_keyword!(lower);
  syn::custom_keyword!(higher);
  syn::custom_keyword!(adjacent);
  syn::custom_keyword!(distinct);
  syn::custom_keyword!(same);
  syn::custom_keyword!(Key);
  syn::custom_keyword!(other);
  syn::custom_keyword!(teams);
  syn::custom_keyword!(player);
}

// ------------------------
// Parsing implementations
// ------------------------

impl Parse for Op {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(Token![+]) {
        input.parse::<Token![+]>()?;

        return Ok(Op::Plus)
      }
      if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;

        return Ok(Op::Minus)
      }
      if input.peek(Token![/]) {
        input.parse::<Token![/]>()?;

        return Ok(Op::Div)
      }
      if input.peek(Token![*]) {
        input.parse::<Token![*]>()?;

        return Ok(Op::Mul)
      }
      if input.peek(Token![%]) {
        input.parse::<Token![%]>()?;

        return Ok(Op::Mod)
      }
      
      Err(input.error("Op could not be parsed!"))
  }
}

impl Parse for IntCmpOp {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(Token![==]) {
        input.parse::<Token![==]>()?;

        return Ok(IntCmpOp::Eq)
      }
      if input.peek(Token![!=]) {
        input.parse::<Token![!=]>()?;

        return Ok(IntCmpOp::Neq)
      }
      if input.peek(Token![<=]) {
        input.parse::<Token![<=]>()?;

        return Ok(IntCmpOp::Le)
      }
      if input.peek(Token![>=]) {
        input.parse::<Token![>=]>()?;

        return Ok(IntCmpOp::Ge)
      }
      if input.peek(Token![<]) {
        input.parse::<Token![<]>()?;

        return Ok(IntCmpOp::Lt)
      }
      if input.peek(Token![>]) {
        input.parse::<Token![>]>()?;

        return Ok(IntCmpOp::Gt)
      }
      
      Err(input.error("IntCmpOp could not be parsed!"))
  }
}

impl Parse for Status {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::face) && input.peek2(kw::down) {
        input.parse::<kw::face>()?;
        input.parse::<kw::down>()?;

        return Ok(Status::FaceDown)
      }
      if input.peek(kw::face) && input.peek2(kw::up) {
        input.parse::<kw::face>()?;
        input.parse::<kw::up>()?;

        return Ok(Status::FaceUp)
      }
      if input.peek(kw::private) {
        input.parse::<kw::private>()?;

        return Ok(Status::Private)
      }
      
      Err(input.error("Status could not be parsed!"))
  }
}

impl Parse for Quantifier {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::all) {
        input.parse::<kw::all>()?;

        return Ok(Quantifier::All)
      }
      if input.peek(kw::any) {
        input.parse::<kw::any>()?;

        return Ok(Quantifier::Any)
      }

      Err(input.error("Quantifier could not be parsed!"))
  }
}

impl Parse for PlayerExpr {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::others) {

        return Err(input.error("PlayerExpr can not be KeyWord 'others'!"))
      }
      if input.peek(kw::current) {
        input.parse::<kw::current>()?;
        return Ok(PlayerExpr::Current)
      }
      if input.peek(kw::previous) {
        input.parse::<kw::previous>()?;
        return Ok(PlayerExpr::Previous)
      }
      if input.peek(kw::competitor) {
        input.parse::<kw::competitor>()?;
        return Ok(PlayerExpr::Competitor)
      }
      if input.peek(kw::owner) && input.peek2(kw::of) && input.peek3(kw::highest)  {
        input.parse::<kw::owner>()?;
        input.parse::<kw::of>()?;
        input.parse::<kw::highest>()?;

        let memory = input.parse::<Ident>()?;

        return Ok(PlayerExpr::OwnerOfHighest(memory.to_string()))
      }
      if input.peek(kw::owner) && input.peek2(kw::of) && input.peek3(kw::lowest)  {
        input.parse::<kw::owner>()?;
        input.parse::<kw::of>()?;
        input.parse::<kw::lowest>()?;

        let memory = input.parse::<Ident>()?;

        return Ok(PlayerExpr::OwnerOfLowest(memory.to_string()))
      }
      if input.peek(kw::turnorder)  {
        input.parse::<kw::turnorder>()?;
        
        let content;
        parenthesized!(content in input);

        let int = content.parse::<IntExpr>()?;

        return Ok(PlayerExpr::Turnorder(int))
      }
      if input.peek(kw::owner) && input.peek(kw::of)   {
        input.parse::<kw::owner>()?;
        input.parse::<kw::of>()?;
        
        let cardpos = input.parse::<CardPosition>()?;

        return Ok(PlayerExpr::OwnerOf(Box::new(cardpos)))
      }

      let playername = input.parse::<Ident>()?;
      
      return Ok(PlayerExpr::PlayerName(playername.to_string()))
  }
}

impl Parse for TeamExpr {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::team) && input.peek2(kw::of) {
        input.parse::<kw::team>()?;
        input.parse::<kw::of>()?;

        let player = input.parse::<PlayerExpr>()?;

        return Ok(TeamExpr::TeamOf(player))
      }

      let teamname = input.parse::<Ident>()?;
      
      return Ok(TeamExpr::TeamName(teamname.to_string()))
  }
}

impl Parse for LocationExpr {
  fn parse(input: ParseStream) -> Result<Self> {
      let id = (input.parse::<Ident>()?).to_string();

      if input.peek(kw::of) && input.peek2(kw::team) {
        input.parse::<kw::of>()?;
        input.parse::<kw::team>()?;
        
        let content;
        parenthesized!(content in input);

        let team = content.parse::<TeamExpr>()?;

        return Ok(LocationExpr::LocationTeam(id, team))
      }
      if input.peek(kw::of) && input.peek2(kw::player) {
        input.parse::<kw::of>()?;
        input.parse::<kw::player>()?;

        let content;
        parenthesized!(content in input);

        let player = content.parse::<PlayerExpr>()?;

        return Ok(LocationExpr::LocationPlayer(id, player))
      }
      
      return Ok(LocationExpr::LocationCurrentOrTable(id))
  }
}

impl Parse for CardPosition {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::top) && input.peek2(kw::of) {
        input.parse::<kw::top>()?;
        input.parse::<kw::of>()?;

        let location = input.parse::<LocationExpr>()?;

        return Ok(CardPosition::Top(location))
      }
      if input.peek(kw::bottom) && input.peek2(kw::of) {
        input.parse::<kw::bottom>()?;
        input.parse::<kw::of>()?;

        let location = input.parse::<LocationExpr>()?;

        return Ok(CardPosition::Bottom(location))
      }
      if input.peek(kw::max) && input.peek2(kw::of) {
        input.parse::<kw::max>()?;
        input.parse::<kw::of>()?;

        let cardset = input.parse::<CardSet>()?;

        if input.peek(kw::using) && input.peek2(kw::prec) {
          input.parse::<kw::using>()?;
          input.parse::<kw::prec>()?;

          let content;
          parenthesized!(content in input);

          let prec_id = (content.parse::<Ident>()?).to_string();

          return Ok(CardPosition::MaxPrec(Box::new(cardset), prec_id))
        }

        if input.peek(kw::using) && input.peek2(kw::point) {
          input.parse::<kw::using>()?;
          input.parse::<kw::point>()?;

          let content;
          parenthesized!(content in input);

          let point_id = (content.parse::<Ident>()?).to_string();

          return Ok(CardPosition::MaxPoint(Box::new(cardset), point_id))
        }

        return Err(input.error("No Precedence or PointMap found to parse!"))
      }
      if input.peek(kw::min) && input.peek2(kw::of) {
        input.parse::<kw::min>()?;
        input.parse::<kw::of>()?;

        let cardset = input.parse::<CardSet>()?;

        if input.peek(kw::using) && input.peek2(kw::prec) {
          input.parse::<kw::using>()?;
          input.parse::<kw::prec>()?;

          let content;
          parenthesized!(content in input);

          let prec_id = (content.parse::<Ident>()?).to_string();

          return Ok(CardPosition::MinPrec(Box::new(cardset), prec_id))
        }

        if input.peek(kw::using) && input.peek2(kw::point) {
          input.parse::<kw::using>()?;
          input.parse::<kw::point>()?;

          let content;
          parenthesized!(content in input);

          let point_id = (content.parse::<Ident>()?).to_string();

          return Ok(CardPosition::MinPoint(Box::new(cardset), point_id))
        }

        return Err(input.error("No Precedence or PointMap found to parse!"))
      }
      
      let location = input.parse::<LocationExpr>()?;
      
      let content;
      bracketed!(content in input);
      let int = content.parse::<IntExpr>()?;

      return Ok(CardPosition::At(location, int))
  }
}

impl Parse for IntExpr {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let left = content.parse::<IntExpr>()?;
        let op = content.parse::<Op>()?;
        let right = content.parse::<IntExpr>()?;

        return Ok(IntExpr::IntOp(Box::new(left), op, Box::new(right)))
      }
      if input.peek(kw::size) && input.peek2(kw::of) {
        input.parse::<kw::size>()?;
        input.parse::<kw::of>()?;

        let collection = input.parse::<Collection>()?;

        return Ok(IntExpr::SizeOf(collection))
      }
      if input.peek(kw::sum) && input.peek2(kw::of) {
        input.parse::<kw::sum>()?;
        input.parse::<kw::of>()?;

        let cardset = input.parse::<CardSet>()?;
        input.parse::<kw::using>()?;
        let point_id = (input.parse::<Ident>()?).to_string();

        return Ok(IntExpr::SumOfCardSet(Box::new(cardset), point_id))
      }
      if input.peek(kw::sum) {
        input.parse::<kw::sum>()?;

        let intcollection = input.parse::<IntCollection>()?;
        return Ok(IntExpr::SumOfIntCollection(intcollection))
      }
      if input.peek(kw::min) && input.peek2(kw::of) {
        input.parse::<kw::min>()?;
        input.parse::<kw::of>()?;

        let cardset = input.parse::<CardSet>()?;
        input.parse::<kw::using>()?;
        let point_id = (input.parse::<Ident>()?).to_string();

        return Ok(IntExpr::MinOf(Box::new(cardset), point_id))
      }
      if input.peek(kw::max) && input.peek2(kw::of) {
        input.parse::<kw::max>()?;
        input.parse::<kw::of>()?;

        let cardset = input.parse::<CardSet>()?;
        input.parse::<kw::using>()?;
        let point_id = (input.parse::<Ident>()?).to_string();

        return Ok(IntExpr::MaxOf(Box::new(cardset), point_id))
      }
      if input.peek(kw::min) {
        input.parse::<kw::min>()?;

        let intcollection = input.parse::<IntCollection>()?;

        return Ok(IntExpr::MinIntCollection(intcollection))
      }
      if input.peek(kw::max) {
        input.parse::<kw::max>()?;

        let intcollection = input.parse::<IntCollection>()?;

        return Ok(IntExpr::MaxIntCollection(intcollection))
      }
      if input.peek(kw::stageroundcounter) {
        input.parse::<kw::stageroundcounter>()?;

        return Ok(IntExpr::StageRoundCounter)
      }

      let int: i32 = (input.parse::<LitInt>()?).base10_parse()?;
      
      return Ok(IntExpr::Int(int))
  }
}

// BoolExpr ==================================================================
impl Parse for BoolExpr {
  fn parse(input: ParseStream) -> Result<Self> {
        parse_with_alternatives(input, &[
            parse_not,
            parse_paren_bool_and,
            parse_paren_bool_or,
            parse_string_compare_eq,
            parse_string_compare_neq,
            parse_int_compare,
            parse_cardset_compare_eq,
            parse_cardset_compare_neq,
            parse_cardset_empty,
            parse_player_compare_eq,
            parse_player_compare_neq,
            parse_team_compare_eq,
            parse_team_compare_neq,
            parse_outof_player,
            parse_outof_collection,
        ])
    }
}

fn parse_with_alternatives<T>(input: ParseStream, alts: &[fn(ParseStream) -> Result<T>]) -> Result<T> {
    for alt in alts {
        let fork = input.fork();
        if let Ok(result) = alt(&fork) {
            input.advance_to(&fork);
            return Ok(result);
        }
    }
    Err(input.error("no alternative matched"))
}

fn parse_not(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::not>()?;
    let bool_expr = input.parse::<BoolExpr>()?;

    Ok(BoolExpr::Not(Box::new(bool_expr)))
}

fn parse_paren_bool_and(input: ParseStream) -> Result<BoolExpr> {
    let content;
    parenthesized!(content in input);

    let left = content.parse::<BoolExpr>()?;
    content.parse::<kw::and>()?;
    let right = content.parse::<BoolExpr>()?;

    Ok(BoolExpr::And(Box::new(left), Box::new(right)))
}

fn parse_paren_bool_or(input: ParseStream) -> Result<BoolExpr> {
    let content;
    parenthesized!(content in input);

    let left = content.parse::<BoolExpr>()?;
    content.parse::<kw::or>()?;
    let right = content.parse::<BoolExpr>()?;

    Ok(BoolExpr::Or(Box::new(left), Box::new(right)))
}

fn parse_string_compare_eq(input: ParseStream) -> Result<BoolExpr> {
    let left = input.parse::<StringExpr>()?;
    input.parse::<Token![==]>()?;
    let right = input.parse::<StringExpr>()?;

    Ok(BoolExpr::StringEq(left, right))
}

fn parse_string_compare_neq(input: ParseStream) -> Result<BoolExpr> {
    let left = input.parse::<StringExpr>()?;
    input.parse::<Token![!=]>()?;
    let right = input.parse::<StringExpr>()?;

    Ok(BoolExpr::StringNeq(left, right))
}

fn parse_int_compare(input: ParseStream) -> Result<BoolExpr> {
    let left = input.parse::<IntExpr>()?;
    let op = input.parse::<IntCmpOp>()?;
    let right = input.parse::<IntExpr>()?;

    return Ok(BoolExpr::IntCmp(left, op, right))
}

fn parse_cardset_compare_eq(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::cards>()?;
    
    let content;
    parenthesized!(content in input);

    let left = content.parse::<CardSet>()?;
    content.parse::<Token![==]>()?;
    let right = content.parse::<CardSet>()?;

    return Ok(BoolExpr::CardSetEq(left, right))
}

fn parse_cardset_compare_neq(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::cards>()?;
    
    let content;
    parenthesized!(content in input);

    let left = content.parse::<CardSet>()?;
    content.parse::<Token![!=]>()?;
    let right = content.parse::<CardSet>()?;

    return Ok(BoolExpr::CardSetNeq(left, right))
}

fn parse_cardset_empty(input: ParseStream) -> Result<BoolExpr> {
    let cardset = input.parse::<CardSet>()?;
    input.parse::<kw::is>()?;

    if input.peek(kw::not) {
        input.parse::<kw::not>()?;
        input.parse::<kw::empty>()?;

        return Ok(BoolExpr::CardSetIsNotEmpty(cardset))
    }

    input.parse::<kw::empty>()?;

    return Ok(BoolExpr::CardSetIsEmpty(cardset))
}

fn parse_player_compare_eq(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::player>()?;
    
    let content;
    parenthesized!(content in input);

    let left = content.parse::<PlayerExpr>()?;
    content.parse::<Token![==]>()?;
    let right = content.parse::<PlayerExpr>()?;

    return Ok(BoolExpr::PlayerEq(left, right))
}

fn parse_player_compare_neq(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::player>()?;
    
    let content;
    parenthesized!(content in input);

    let left = content.parse::<PlayerExpr>()?;
    content.parse::<Token![!=]>()?;
    let right = content.parse::<PlayerExpr>()?;

    return Ok(BoolExpr::PlayerNeq(left, right))
}

fn parse_team_compare_eq(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::team>()?;
    
    let content;
    parenthesized!(content in input);

    let left = content.parse::<TeamExpr>()?;
    content.parse::<Token![==]>()?;
    let right = content.parse::<TeamExpr>()?;

    return Ok(BoolExpr::TeamEq(left, right))
}

fn parse_team_compare_neq(input: ParseStream) -> Result<BoolExpr> {
    input.parse::<kw::team>()?;
    
    let content;
    parenthesized!(content in input);

    let left = content.parse::<TeamExpr>()?;
    content.parse::<Token![!=]>()?;
    let right = content.parse::<TeamExpr>()?;

    return Ok(BoolExpr::TeamNeq(left, right))
}

fn parse_outof_player(input: ParseStream) -> Result<BoolExpr> {
    let player = input.parse::<PlayerExpr>()?;
    input.parse::<kw::out>()?;
    input.parse::<kw::of>()?;

    if input.peek(kw::stage) {
      input.parse::<kw::stage>()?;

      return Ok(BoolExpr::OutOfStagePlayer(player))
    }
    if input.peek(kw::game) {
      input.parse::<kw::game>()?;

      return Ok(BoolExpr::OutOfGamePlayer(player))
    }

    Err(input.error("Could not parse 'out of'!"))
}

fn parse_outof_collection(input: ParseStream) -> Result<BoolExpr> {
    let player_collection = input.parse::<PlayerCollection>()?;
    input.parse::<kw::out>()?;
    input.parse::<kw::of>()?;

    if input.peek(kw::stage) {
      input.parse::<kw::stage>()?;

      return Ok(BoolExpr::OutOfStageCollection(player_collection))
    }

    if input.peek(kw::game) {
      input.parse::<kw::game>()?;

      return Ok(BoolExpr::OutOfGameCollection(player_collection))
    }

    Err(input.error("Could not parse 'out of'!"))
}

// ===========================================================================

// StringExpr ================================================================
impl Parse for StringExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(expr) = try_parse_keyof(input) {
            return Ok(expr);
        }

        if let Ok(expr) = try_parse_collection_at(input) {
            return Ok(expr);
        }

        let id = input.parse::<Ident>()?;
        Ok(StringExpr::ID(id.to_string()))
    }
}

fn try_parse_keyof(input: ParseStream) -> Result<StringExpr> {
    let fork = input.fork();

    let key: Ident = match fork.parse::<Ident>() {
        Ok(k) => k,
        Err(_) => return Err(input.error("not a KeyOf expression")),
    };

    fork.parse::<kw::of>()?;

    let position: CardPosition = match fork.parse() {
        Ok(p) => p,
        Err(_) => return Err(input.error("missing CardPosition")),
    };

    input.advance_to(&fork);
    Ok(StringExpr::KeyOf(key.to_string(), position))
}

fn try_parse_collection_at(input: ParseStream) -> Result<StringExpr> {
    let fork = input.fork();

    let string_collection: StringCollection = match fork.parse::<StringCollection>() {
      Ok(s) => s,
      Err(_) => return Err(input.error("missing StringCollection")),
    };    

    let content;
    bracketed!(content in fork);

    let int: IntExpr = content.parse()?;

    input.advance_to(&fork);
    Ok(StringExpr::StringCollectionAt(string_collection, int))
}

// ===========================================================================

impl Parse for PlayerCollection {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::others) {
        input.parse::<kw::others>()?;

        return Ok(PlayerCollection::Others)
      }
      if input.peek(kw::playersin) {
        input.parse::<kw::playersin>()?;

        return Ok(PlayerCollection::PlayersIn)
      }
      if input.peek(kw::playersout) {
        input.parse::<kw::playersout>()?;

        return Ok(PlayerCollection::PlayersOut)
      }
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let players: Punctuated<PlayerExpr, Token![,]> =
            content.parse_terminated(PlayerExpr::parse, Token![,])?;

        return Ok(PlayerCollection::Player(players.into_iter().collect()))
      }
      
      let quantifier = input.parse::<Quantifier>()?;
      return Ok(PlayerCollection::Quantifier(quantifier))
  }
}

impl Parse for FilterExpr {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::same) {
        input.parse::<kw::same>()?;

        let key = (input.parse::<Ident>()?).to_string();

        return Ok(FilterExpr::Same(key))
      }
      if input.peek(kw::distinct) {
        input.parse::<kw::distinct>()?;

        let key = (input.parse::<Ident>()?).to_string();

        return Ok(FilterExpr::Distinct(key))
      }
      if input.peek(kw::adjacent) {
        input.parse::<kw::adjacent>()?;

        let key = (input.parse::<Ident>()?).to_string();
        input.parse::<kw::using>()?;
        let prec = (input.parse::<Ident>()?).to_string();

        return Ok(FilterExpr::Adjacent(key, prec))
      }
      if input.peek(kw::higher) {
        input.parse::<kw::higher>()?;

        let key = (input.parse::<Ident>()?).to_string();
        input.parse::<kw::using>()?;
        let prec = (input.parse::<Ident>()?).to_string();

        return Ok(FilterExpr::Higher(key, prec))
      }
      if input.peek(kw::lower) {
        input.parse::<kw::lower>()?;

        let key = (input.parse::<Ident>()?).to_string();
        input.parse::<kw::using>()?;
        let prec = (input.parse::<Ident>()?).to_string();

        return Ok(FilterExpr::Lower(key, prec))
      }
      if input.peek(kw::size) {
        input.parse::<kw::size>()?;

        if input.peek(Token![==]) {
          input.parse::<Token![==]>()?;
          let int = input.parse::<IntExpr>()?;

          return Ok(FilterExpr::SizeEq(Box::new(int)))
        }
        if input.peek(Token![!=]) {
          input.parse::<Token![!=]>()?;
          let int = input.parse::<IntExpr>()?;

          return Ok(FilterExpr::SizeNeq(Box::new(int)))
        }
        if input.peek(Token![<=]) {
          input.parse::<Token![<=]>()?;
          let int = input.parse::<IntExpr>()?;

          return Ok(FilterExpr::SizeLe(Box::new(int)))
        }
        if input.peek(Token![>=]) {
          input.parse::<Token![>=]>()?;
          let int = input.parse::<IntExpr>()?;

          return Ok(FilterExpr::SizeGe(Box::new(int)))
        }

        if input.peek(Token![<]) {
          input.parse::<Token![<]>()?;
          let int = input.parse::<IntExpr>()?;

          return Ok(FilterExpr::SizeLt(Box::new(int)))
        }
        if input.peek(Token![>]) {
          input.parse::<Token![>]>()?;
          let int = input.parse::<IntExpr>()?;

          return Ok(FilterExpr::SizeGt(Box::new(int)))
        }

        return Err(input.error("FilterExpr for 'size' could not been parsed!"))
      }
      if input.peek(kw::Key) {
        input.parse::<kw::Key>()?;

        let content;
        parenthesized!(content in input);

        let key = (content.parse::<Ident>()?).to_string();

        if content.peek(Token![==]) {
          content.parse::<Token![==]>()?;

          let string = content.parse::<StringExpr>()?;

          return Ok(FilterExpr::KeyEq(key, Box::new(string)))
        }
        if content.peek(Token![!=]) {
          content.parse::<Token![!=]>()?;

          let string = content.parse::<StringExpr>()?;

          return Ok(FilterExpr::KeyNeq(key, Box::new(string)))
        }
        
        return Err(input.error("FilterExpr for 'Key Comparison' could not be parsed!"))
      }
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let filter_left = content.parse::<FilterExpr>()?;
        
        if content.peek(kw::and) {
          content.parse::<kw::and>()?;

          let filter_right = content.parse::<FilterExpr>()?;

          return Ok(FilterExpr::And(Box::new(filter_left), Box::new(filter_right)))
        }
        if content.peek(kw::or) {
          content.parse::<kw::or>()?;

          let filter_right = content.parse::<FilterExpr>()?;

          return Ok(FilterExpr::Or(Box::new(filter_left), Box::new(filter_right)))
        }
      }
      if input.peek(kw::not) {
        input.parse::<kw::not>()?;

        let combo = (input.parse::<Ident>()?).to_string();

        return Ok(FilterExpr::NotCombo(combo))
      }

      // last rule to match is Combo
      let combo = (input.parse::<Ident>()?).to_string();

      return Ok(FilterExpr::Combo(combo))
  }
}

impl Parse for Group {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(pos) = fork.parse::<CardPosition>() {
            input.advance_to(&fork);
            return Ok(Group::CardPosition(pos));
        }

        let fork = input.fork();
        if let Ok(combo) = fork.parse::<Ident>() {
            if fork.peek(kw::not) {
              fork.parse::<kw::not>()?;
              fork.parse::<Token![in]>()?;
              if let Ok(loc) = fork.parse::<LocationExpr>() {
                  input.advance_to(&fork);
                  return Ok(Group::NotComboInLocation(combo.to_string(), loc));
              }
              if let Ok(locs) = fork.parse::<LocationCollection>() {
                  input.advance_to(&fork);
                  return Ok(Group::NotComboInLocationCollection(combo.to_string(), locs));
              }
            }
            if fork.peek(Token![in]) {
                fork.parse::<Token![in]>()?;
                if let Ok(loc) = fork.parse::<LocationExpr>() {
                    input.advance_to(&fork);
                    return Ok(Group::ComboInLocation(combo.to_string(), loc));
                }
                if let Ok(locs) = fork.parse::<LocationCollection>() {
                    input.advance_to(&fork);
                    return Ok(Group::ComboInLocationCollection(combo.to_string(), locs));
                }
            }
        }

        let fork = input.fork();
        if let Ok(locs) = fork.parse::<LocationCollection>() {
            if fork.peek(Token![where]) {
                fork.parse::<Token![where]>()?;
                let filter: FilterExpr = fork.parse()?;
                input.advance_to(&fork);
                return Ok(Group::LocationCollectionWhere(locs, filter));
            }
        }

        let fork = input.fork();
        if let Ok(locs) = fork.parse::<LocationCollection>() {
            input.advance_to(&fork);
            return Ok(Group::LocationCollection(locs));
        }

        let fork = input.fork();
        if let Ok(loc) = fork.parse::<LocationExpr>() {
            if fork.peek(Token![where]) {
                fork.parse::<Token![where]>()?;
                let filter: FilterExpr = fork.parse()?;
                input.advance_to(&fork);
                return Ok(Group::LocationWhere(loc, filter));
            }
        }

        let fork = input.fork();
        if let Ok(loc) = fork.parse::<LocationExpr>() {
            input.advance_to(&fork);
            return Ok(Group::Location(loc));
        }

        Err(input.error("expected a Group expression"))
    }
}

impl Parse for CardSet {
  fn parse(input: ParseStream) -> Result<Self> {
      let group = input.parse::<Group>()?;

      if input.peek(kw::of) {
        input.parse::<kw::of>()?;
        let fork = input.fork();
        if let Ok(playercollection) = fork.parse::<PlayerCollection>() {
          input.advance_to(&fork);

          return Ok(CardSet::GroupOfPlayerCollection(group, playercollection))
        }

        let fork = input.fork();
        if let Ok(player) = fork.parse::<PlayerExpr>() {
          input.advance_to(&fork);

          return Ok(CardSet::GroupOfPlayer(group, player))
        }

        return Err(input.error("CardSet could not been parsed! Missing Player or PlayerCollection after 'of'"))
      }

      return Ok(CardSet::Group(group))
  }
}

impl Parse for IntCollection {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let ints: Punctuated<IntExpr, Token![,]> =
            content.parse_terminated(IntExpr::parse, Token![,])?;

        return Ok(IntCollection { ints: ints.into_iter().collect() })
      }

      Err(input.error("No IntCollection found to parse!"))
  }
}

impl Parse for LocationCollection {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let locations: Punctuated<LocationExpr, Token![,]> =
            content.parse_terminated(LocationExpr::parse, Token![,])?;

        return Ok(LocationCollection { locations: locations.into_iter().collect() })
      }

      Err(input.error("No LocationCollection found to parse!"))
  }
}

impl Parse for TeamCollection {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::other) && input.peek2(kw::teams) {
        input.parse::<kw::other>()?;
        input.parse::<kw::teams>()?;

        return Ok(TeamCollection::OtherTeams)
      }
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let teams: Punctuated<TeamExpr, Token![,]> =
            content.parse_terminated(TeamExpr::parse, Token![,])?;

        return Ok(TeamCollection::Team(teams.into_iter().collect()))
      }

      Err(input.error("No TeamCollection found to parse!"))
  }
}

impl Parse for StringCollection {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);

        let strings: Punctuated<StringExpr, Token![,]> =
            content.parse_terminated(StringExpr::parse, Token![,])?;

        return Ok(StringCollection { strings: strings.into_iter().collect() })
      }

      Err(input.error("No StringCollection found to parse!"))
  }
}

impl Parse for Collection {
  fn parse(input: ParseStream) -> Result<Self> {
      let fork = input.fork();
      if let Ok(playercollection) = fork.parse::<PlayerCollection>() {
        input.advance_to(&fork);

        return Ok(Collection::PlayerCollection(playercollection))
      }

      let fork = input.fork();
      if let Ok(intcollection) = fork.parse::<IntCollection>() {
        input.advance_to(&fork);

        return Ok(Collection::IntCollection(intcollection))
      }
      
      let fork = input.fork();
      if let Ok(stringcollection) = fork.parse::<StringCollection>() {
        input.advance_to(&fork);

        return Ok(Collection::StringCollection(stringcollection))
      }
      
      let fork = input.fork();
      if let Ok(locationcollection) = fork.parse::<LocationCollection>() {
        input.advance_to(&fork);

        return Ok(Collection::LocationCollection(locationcollection))
      }
      
      let fork = input.fork();
      if let Ok(teamcollection) = fork.parse::<TeamCollection>() {
        input.advance_to(&fork);

        return Ok(Collection::TeamCollection(teamcollection))
      }

      let fork = input.fork();
      if let Ok(cardset) = fork.parse::<CardSet>() {
        input.advance_to(&fork);

        return Ok(Collection::CardSet(Box::new(cardset)))
      }
      
      Err(input.error("No Collection found to Parse!"))
  }
}

