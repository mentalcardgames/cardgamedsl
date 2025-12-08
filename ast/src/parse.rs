use crate::ast::*;

use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{LitInt, Token, braced, bracketed, parenthesized};


// ------------------------
// Keywords declarations
// ------------------------

mod kw {
  syn::custom_keyword!(choose);
  syn::custom_keyword!(optional);
  syn::custom_keyword!(next);
  syn::custom_keyword!(turn);
  syn::custom_keyword!(winner);
  syn::custom_keyword!(demand);
  syn::custom_keyword!(cycle);
  syn::custom_keyword!(bid);
  syn::custom_keyword!(successful);
  syn::custom_keyword!(fail);
  syn::custom_keyword!(set);
  syn::custom_keyword!(shuffle);
  syn::custom_keyword!(flip);
  syn::custom_keyword!(combo);
  syn::custom_keyword!(memory);
  syn::custom_keyword!(pointmap);
  syn::custom_keyword!(precedence);
  syn::custom_keyword!(token);
  syn::custom_keyword!(random);
  syn::custom_keyword!(location);
  syn::custom_keyword!(table);
  syn::custom_keyword!(on);
  syn::custom_keyword!(card);
  syn::custom_keyword!(with);
  syn::custom_keyword!(place);
  syn::custom_keyword!(exchange);
  syn::custom_keyword!(deal);
  syn::custom_keyword!(range);
  syn::custom_keyword!(from);
  syn::custom_keyword!(to);
  syn::custom_keyword!(until);
  syn::custom_keyword!(end);
  syn::custom_keyword!(times);
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
  syn::custom_keyword!(locations);
  syn::custom_keyword!(ints);
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
      if input.peek(kw::next) {
        input.parse::<kw::next>()?;
        return Ok(PlayerExpr::Next)
      }
      if input.peek(kw::competitor) {
        input.parse::<kw::competitor>()?;
        return Ok(PlayerExpr::Competitor)
      }
      if input.peek(kw::owner) && input.peek2(kw::of) && input.peek3(kw::highest)  {
        input.parse::<kw::owner>()?;
        input.parse::<kw::of>()?;
        input.parse::<kw::highest>()?;

        let memory = input.parse::<Memory>()?;

        return Ok(PlayerExpr::OwnerOfHighest(memory))
      }
      if input.peek(kw::owner) && input.peek2(kw::of) && input.peek3(kw::lowest)  {
        input.parse::<kw::owner>()?;
        input.parse::<kw::of>()?;
        input.parse::<kw::lowest>()?;

        let memory = input.parse::<Memory>()?;

        return Ok(PlayerExpr::OwnerOfLowest(memory))
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

      let playername = input.parse::<PlayerName>()?;
      
      return Ok(PlayerExpr::PlayerName(playername))
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

      let teamname = input.parse::<TeamName>()?;
      
      return Ok(TeamExpr::TeamName(teamname))
  }
}

impl Parse for CardPosition {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::top) && input.peek2(kw::of) {
        input.parse::<kw::top>()?;
        input.parse::<kw::of>()?;

        let location = input.parse::<Location>()?;

        return Ok(CardPosition::Top(location))
      }
      if input.peek(kw::bottom) && input.peek2(kw::of) {
        input.parse::<kw::bottom>()?;
        input.parse::<kw::of>()?;

        let location = input.parse::<Location>()?;

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

          let precedence = content.parse::<Precedence>()?;

          return Ok(CardPosition::MaxPrec(Box::new(cardset), precedence))
        }

        if input.peek(kw::using) && input.peek2(kw::point) {
          input.parse::<kw::using>()?;
          input.parse::<kw::point>()?;

          let content;
          parenthesized!(content in input);

          let pointmap = content.parse::<PointMap>()?;

          return Ok(CardPosition::MaxPoint(Box::new(cardset), pointmap))
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

          let precedence = content.parse::<Precedence>()?;

          return Ok(CardPosition::MinPrec(Box::new(cardset), precedence))
        }

        if input.peek(kw::using) && input.peek2(kw::point) {
          input.parse::<kw::using>()?;
          input.parse::<kw::point>()?;

          let content;
          parenthesized!(content in input);

          let pointmap = content.parse::<PointMap>()?;

          return Ok(CardPosition::MinPoint(Box::new(cardset), pointmap))
        }

        return Err(input.error("No Precedence or PointMap found to parse!"))
      }
      
      let location = input.parse::<Location>()?;
      
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
        let pointmap = input.parse::<PointMap>()?;

        return Ok(IntExpr::SumOfCardSet(Box::new(cardset), pointmap))
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
        let pointmap = input.parse::<PointMap>()?;

        return Ok(IntExpr::MinOf(Box::new(cardset), pointmap))
      }
      if input.peek(kw::max) && input.peek2(kw::of) {
        input.parse::<kw::max>()?;
        input.parse::<kw::of>()?;

        let cardset = input.parse::<CardSet>()?;
        input.parse::<kw::using>()?;
        let pointmap = input.parse::<PointMap>()?;

        return Ok(IntExpr::MaxOf(Box::new(cardset), pointmap))
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

        let id = input.parse::<ID>()?;
        Ok(StringExpr::ID(id))
    }
}

fn try_parse_keyof(input: ParseStream) -> Result<StringExpr> {
    let fork = input.fork();

    let key = match fork.parse::<Key>() {
        Ok(k) => k,
        Err(_) => return Err(input.error("not a KeyOf expression")),
    };

    fork.parse::<kw::of>()?;

    let position = match fork.parse::<CardPosition>() {
        Ok(p) => p,
        Err(_) => return Err(input.error("missing CardPosition")),
    };

    input.advance_to(&fork);

    Ok(StringExpr::KeyOf(key, position))
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

        let key = input.parse::<Key>()?;

        return Ok(FilterExpr::Same(key))
      }
      if input.peek(kw::distinct) {
        input.parse::<kw::distinct>()?;

        let key = input.parse::<Key>()?;

        return Ok(FilterExpr::Distinct(key))
      }
      if input.peek(kw::adjacent) {
        input.parse::<kw::adjacent>()?;

        let key = input.parse::<Key>()?;
        input.parse::<kw::using>()?;
        let precedence = input.parse::<Precedence>()?;

        return Ok(FilterExpr::Adjacent(key, precedence))
      }
      if input.peek(kw::higher) {
        input.parse::<kw::higher>()?;

        let key = input.parse::<Key>()?;
        input.parse::<kw::using>()?;
        let precedence = input.parse::<Precedence>()?;

        return Ok(FilterExpr::Higher(key, precedence))
      }
      if input.peek(kw::lower) {
        input.parse::<kw::lower>()?;

        let key = input.parse::<Key>()?;
        input.parse::<kw::using>()?;
        let precedence = input.parse::<Precedence>()?;

        return Ok(FilterExpr::Lower(key, precedence))
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

        let key = content.parse::<Key>()?;

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

        let combo = input.parse::<Combo>()?;

        return Ok(FilterExpr::NotCombo(combo))
      }

      // last rule to match is Combo
      let combo = input.parse::<Combo>()?;

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
        if let Ok(combo) = fork.parse::<Combo>() {
            if fork.peek(kw::not) {
              fork.parse::<kw::not>()?;
              fork.parse::<Token![in]>()?;
              if let Ok(loc) = fork.parse::<Location>() {
                  input.advance_to(&fork);
                  return Ok(Group::NotComboInLocation(combo, loc));
              }
              if let Ok(locs) = fork.parse::<LocationCollection>() {
                  input.advance_to(&fork);
                  return Ok(Group::NotComboInLocationCollection(combo, locs));
              }
            }
            if fork.peek(Token![in]) {
                fork.parse::<Token![in]>()?;
                if let Ok(loc) = fork.parse::<Location>() {
                    input.advance_to(&fork);
                    return Ok(Group::ComboInLocation(combo, loc));
                }
                if let Ok(locs) = fork.parse::<LocationCollection>() {
                    input.advance_to(&fork);
                    return Ok(Group::ComboInLocationCollection(combo, locs));
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
        if let Ok(loc) = fork.parse::<Location>() {
            if fork.peek(Token![where]) {
                fork.parse::<Token![where]>()?;
                let filter: FilterExpr = fork.parse()?;
                input.advance_to(&fork);
                return Ok(Group::LocationWhere(loc, filter));
            }
        }

        let fork = input.fork();
        if let Ok(loc) = fork.parse::<Location>() {
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

        let locations: Punctuated<Location, Token![,]> =
            content.parse_terminated(Location::parse, Token![,])?;

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
    if input.peek(kw::players) {
      input.parse::<kw::players>()?;

      let playercollection = input.parse::<PlayerCollection>()?;

      return Ok(Collection::PlayerCollection(playercollection))
    }
    if input.peek(kw::teams) {
      input.parse::<kw::teams>()?;

      let teamcollection = input.parse::<TeamCollection>()?;

      return Ok(Collection::TeamCollection(teamcollection))
    }
    if input.peek(kw::ints) {
      input.parse::<kw::ints>()?;

      let intcollection = input.parse::<IntCollection>()?;

      return Ok(Collection::IntCollection(intcollection))
    }
    if input.peek(kw::ints) {
      input.parse::<kw::ints>()?;

      let intcollection = input.parse::<IntCollection>()?;

      return Ok(Collection::IntCollection(intcollection))
    }
    if input.peek(kw::locations) {
      input.parse::<kw::locations>()?;

      let locationcollection = input.parse::<LocationCollection>()?;

      return Ok(Collection::LocationCollection(locationcollection))
    }
    if input.peek(kw::cards) {
      input.parse::<kw::cards>()?;

      let cardset = input.parse::<CardSet>()?;

      return Ok(Collection::CardSet(Box::new(cardset)))
    }
    

    let stringcollection = input.parse::<StringCollection>()?;

    return Ok(Collection::StringCollection(stringcollection))    
  }
}

// TODO: maybe work out how to do it without ambiguity and no 'types' in the front
// ===========================================================================
// ===========================================================================
// ===========================================================================
// impl Parse for Collection {
//   fn parse(input: ParseStream) -> Result<Self> {
//       let fork = input.fork();
//       if let Ok(playercollection) = fork.parse::<PlayerCollection>() {
//         input.advance_to(&fork);

//         return Ok(Collection::PlayerCollection(playercollection))
//       }

//       let fork = input.fork();
//       if let Ok(intcollection) = fork.parse::<IntCollection>() {
//         input.advance_to(&fork);

//         return Ok(Collection::IntCollection(intcollection))
//       }
      
//       let fork = input.fork();
//       if let Ok(stringcollection) = fork.parse::<StringCollection>() {
//         input.advance_to(&fork);

//         return Ok(Collection::StringCollection(stringcollection))
//       }
      
//       let fork = input.fork();
//       if let Ok(locationcollection) = fork.parse::<LocationCollection>() {
//         input.advance_to(&fork);

//         return Ok(Collection::LocationCollection(locationcollection))
//       }
      
//       let fork = input.fork();
//       if let Ok(teamcollection) = fork.parse::<TeamCollection>() {
//         input.advance_to(&fork);

//         return Ok(Collection::TeamCollection(teamcollection))
//       }

//       let fork = input.fork();
//       if let Ok(cardset) = fork.parse::<CardSet>() {
//         input.advance_to(&fork);

//         return Ok(Collection::CardSet(Box::new(cardset)))
//       }
      
//       Err(input.error("No Collection found to Parse!"))
//   }
// }
// ===========================================================================
// ===========================================================================
// ===========================================================================

impl Parse for Repititions {
  fn parse(input: ParseStream) -> Result<Self> {
      let int = input.parse::<IntExpr>()?;

      input.parse::<kw::times>()?;

      return Ok(Repititions { times: int})
  }
}

impl Parse for EndCondition {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::until>()?;

      if input.peek(kw::end) {
        input.parse::<kw::end>()?;

        return Ok(EndCondition::UntilEnd)
      }

      let fork = input.fork();
      if let Ok(boolexpr) = fork.parse::<BoolExpr>() {
        input.advance_to(&fork);

        if input.peek(kw::and) {
          input.parse::<kw::and>()?;

          let reps = input.parse::<Repititions>()?;

          return Ok(EndCondition::UntilBoolAndRep(boolexpr, reps))
        }
        if input.peek(kw::or) {
          input.parse::<kw::or>()?;

          let reps = input.parse::<Repititions>()?;

          return Ok(EndCondition::UntilBoolOrRep(boolexpr, reps))
        }

        return Ok(EndCondition::UntilBool(boolexpr))
      }

      let fork = input.fork();
      if let Ok(reps) = fork.parse::<Repititions>() {
        input.advance_to(&fork);

        return Ok(EndCondition::UntilRep(reps))
      }

      return Err(input.error("EndCondition could not been parsed!"))
  }
}

impl Parse for IntRange {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::range>()?;
      
      let content;
      parenthesized!(content in input);

      if content.peek(Token![==]) {
        content.parse::<Token![==]>()?;

        let int = content.parse::<IntExpr>()?;

        return Ok(IntRange::Eq(int))
      }
      if content.peek(Token![!=]) {
        content.parse::<Token![!=]>()?;

        let int = content.parse::<IntExpr>()?;

        return Ok(IntRange::Neq(int))
      }
      if content.peek(Token![>=]) {
        content.parse::<Token![>=]>()?;

        let int = content.parse::<IntExpr>()?;

        return Ok(IntRange::Ge(int))
      }
      if content.peek(Token![<=]) {
        content.parse::<Token![<=]>()?;

        let int = content.parse::<IntExpr>()?;

        return Ok(IntRange::Le(int))
      }
      if content.peek(Token![>]) {
        content.parse::<Token![>]>()?;

        let int = content.parse::<IntExpr>()?;

        return Ok(IntRange::Gt(int))
      }
      if content.peek(Token![<]) {
        content.parse::<Token![<]>()?;

        let int = content.parse::<IntExpr>()?;

        return Ok(IntRange::Lt(int))
      }
      
      return Err(input.error("No IntRange could been parsed!"))
  }
}

impl Parse for Quantity {
  fn parse(input: ParseStream) -> Result<Self> {
      let fork = input.fork();
      if let Ok(int) = fork.parse::<IntExpr>() {
        input.advance_to(&fork);

        return Ok(Quantity::Int(int))
      }
      let fork = input.fork();
      if let Ok(intrange) = fork.parse::<IntRange>() {
        input.advance_to(&fork);

        return Ok(Quantity::IntRange(intrange))
      }
      let fork = input.fork();
      if let Ok(quantifier) = fork.parse::<Quantifier>() {
        input.advance_to(&fork);

        return Ok(Quantity::Quantifier(quantifier))
      }
      
      return Err(input.error("Quantity could not been parsed"))
  }
}

impl Parse for ClassicMove {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<Token![move]>()?;

      let fork = input.fork();
      if let Ok(quantity) = fork.parse::<Quantity>() {
        input.advance_to(&fork);

        input.parse::<kw::from>()?;
        let from_cardset = input.parse::<CardSet>()?;
        let status = input.parse::<Status>()?;
        input.parse::<kw::to>()?;
        let to_cardset = input.parse::<CardSet>()?;

        return Ok(ClassicMove::MoveQuantity(quantity, from_cardset, status, to_cardset))
      }

      let from_cardset = input.parse::<CardSet>()?;
      let status = input.parse::<Status>()?;
      input.parse::<kw::to>()?;
      let to_cardset = input.parse::<CardSet>()?;

      return Ok(ClassicMove::Move(from_cardset, status, to_cardset))
  }
}

impl Parse for DealMove {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::deal>()?;

      let fork = input.fork();
      if let Ok(quantity) = fork.parse::<Quantity>() {
        input.advance_to(&fork);

        input.parse::<kw::from>()?;
        let from_cardset = input.parse::<CardSet>()?;
        let status = input.parse::<Status>()?;
        input.parse::<kw::to>()?;
        let to_cardset = input.parse::<CardSet>()?;

        return Ok(DealMove::DealQuantity(quantity, from_cardset, status, to_cardset))
      }

      let from_cardset = input.parse::<CardSet>()?;
      let status = input.parse::<Status>()?;
      input.parse::<kw::to>()?;
      let to_cardset = input.parse::<CardSet>()?;

      return Ok(DealMove::Deal(from_cardset, status, to_cardset))
  }
}

impl Parse for ExchangeMove {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::exchange>()?;

      let fork = input.fork();
      if let Ok(quantity) = fork.parse::<Quantity>() {
        input.advance_to(&fork);

        input.parse::<kw::from>()?;
        
        let from_cardset = input.parse::<CardSet>()?;
        let status = input.parse::<Status>()?;
        input.parse::<kw::with>()?;
        let to_cardset = input.parse::<CardSet>()?;

        return Ok(ExchangeMove::ExchangeQuantity(quantity, from_cardset, status, to_cardset))
      }

      let from_cardset = input.parse::<CardSet>()?;
      let status = input.parse::<Status>()?;
      input.parse::<kw::with>()?;
      let to_cardset = input.parse::<CardSet>()?;

      return Ok(ExchangeMove::Exchange(from_cardset, status, to_cardset))
  }
}

impl Parse for TokenLocExpr {
  fn parse(input: ParseStream) -> Result<Self> {
      let fork = input.fork();
      if let Ok(location) = fork.parse::<Location>() {
        input.advance_to(&fork);
  
        if input.peek(kw::of) {
          input.parse::<kw::of>()?;

          let fork = input.fork();
          if let Ok(player) = fork.parse::<PlayerExpr>() {
            input.advance_to(&fork);
          
            return Ok(TokenLocExpr::LocationPlayer(location, player))
          }
        
          let fork = input.fork();
          if let Ok(playercollection) = fork.parse::<PlayerCollection>() {
            input.advance_to(&fork);
          
            return Ok(TokenLocExpr::LocationPlayerCollection(location, playercollection))
          }
        }

        return Ok(TokenLocExpr::Location(location))
      }

      let fork = input.fork();
      if let Ok(locationcollection) = fork.parse::<LocationCollection>() {
        input.advance_to(&fork);
  
        if input.peek(kw::of) {
          input.parse::<kw::of>()?;

          let fork = input.fork();
          if let Ok(player) = fork.parse::<PlayerExpr>() {
            input.advance_to(&fork);
          
            return Ok(TokenLocExpr::LocationCollectionPlayer(locationcollection, player))
          }
        
          let fork = input.fork();
          if let Ok(playercollection) = fork.parse::<PlayerCollection>() {
            input.advance_to(&fork);
          
            return Ok(TokenLocExpr::LocationCollectionPlayerCollection(locationcollection, playercollection))
          }
        }

        return Ok(TokenLocExpr::LocationCollection(locationcollection))
      }

      return Err(input.error("No TokenLoc found to parse!"))
  }
}

impl Parse for TokenMove {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::place>()?;

      let fork = input.fork();
      if let Ok(quantity) = fork.parse::<Quantity>() {
        input.advance_to(&fork);

        input.parse::<kw::from>()?;
        let from_tokenloc = input.parse::<TokenLocExpr>()?;
        input.parse::<kw::to>()?;
        let to_tokenloc = input.parse::<TokenLocExpr>()?;

        return Ok(TokenMove::PlaceQuantity(quantity, from_tokenloc, to_tokenloc))
      }

      let from_tokenloc = input.parse::<TokenLocExpr>()?;
      input.parse::<kw::to>()?;
      let to_tokenloc = input.parse::<TokenLocExpr>()?;

      return Ok(TokenMove::Place(from_tokenloc, to_tokenloc))
  }
}

impl Parse for Rule {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::players) && input.peek2(Token![:]) {
        input.parse::<kw::players>()?;
        input.parse::<Token![:]>()?;

        let content;
        parenthesized!(content in input);

        let players: Punctuated<PlayerName, Token![,]> =
            content.parse_terminated(PlayerName::parse, Token![,])?;

        return Ok(Rule::CreatePlayer(players.into_iter().collect()))
      }
      if input.peek(kw::team) {
        input.parse::<kw::team>()?;

        let teamname = input.parse::<TeamName>()?;

        input.parse::<Token![:]>()?;

        let content;
        parenthesized!(content in input);

        let players: Punctuated<PlayerName, Token![,]> =
            content.parse_terminated(PlayerName::parse, Token![,])?;

        return Ok(Rule::CreateTeam(teamname, players.into_iter().collect()))
      }
      if input.peek(kw::random) && input.peek2(kw::turnorder) && input.peek3(Token![:]) {
        input.parse::<kw::random>()?;
        input.parse::<kw::turnorder>()?;
        input.parse::<Token![:]>()?;

        let content;
        parenthesized!(content in input);

        let players: Punctuated<PlayerName, Token![,]> =
            content.parse_terminated(PlayerName::parse, Token![,])?;

        return Ok(Rule::CreateTurnorderRandom(players.into_iter().collect()))
      }
      if input.peek(kw::turnorder) && input.peek2(Token![:]) {
        input.parse::<kw::turnorder>()?;
        input.parse::<Token![:]>()?;

        let content;
        parenthesized!(content in input);

        let players: Punctuated<PlayerName, Token![,]> =
            content.parse_terminated(PlayerName::parse, Token![,])?;

        return Ok(Rule::CreateTurnorder(players.into_iter().collect()))
      }
      if input.peek(kw::location) {
        input.parse::<kw::location>()?;

        let fork = input.fork();
        if let Ok(locationcollection) = fork.parse::<LocationCollection>() {
          input.advance_to(&fork);

          input.parse::<kw::on>()?;

          if input.peek(kw::players) {
            input.parse::<kw::players>()?;

            let playercollection = input.parse::<PlayerCollection>()?;
            
            return Ok(
              Rule::CreateLocationCollectionOnPlayerCollection(
                locationcollection, playercollection
              )
            )
          }

          if input.peek(kw::teams) {
            input.parse::<kw::teams>()?;

            let teamcollection = input.parse::<TeamCollection>()?;
            
            return Ok(
              Rule::CreateLocationCollectionOnTeamCollection(
                locationcollection, teamcollection
              )
            )
          }

          input.parse::<kw::table>()?;

          return Ok(Rule::CreateLocationCollectionOnTable(locationcollection))
        }

        let location = input.parse::<Location>()?;

        input.parse::<kw::on>()?;

        if input.peek(kw::players) {
          input.parse::<kw::players>()?;

          let playercollection = input.parse::<PlayerCollection>()?;
          
          return Ok(
            Rule::CreateLocationOnPlayerCollection(
              location, playercollection
            )
          )
        }

        if input.peek(kw::teams) {
          input.parse::<kw::teams>()?;

          let teamcollection = input.parse::<TeamCollection>()?;
          
          return Ok(
            Rule::CreateLocationOnTeamCollection(
              location, teamcollection
            )
          )
        }

        input.parse::<kw::table>()?;

        return Ok(Rule::CreateLocationOnTable(location))
      }
      if input.peek(kw::card) && input.peek2(kw::on) {
        input.parse::<kw::card>()?;
        input.parse::<kw::on>()?;

        let location= input.parse::<Location>()?;
        
      input.parse::<Token![:]>()?;

        let types = input.parse::<Types>()?;

        return Ok(Rule::CreateCardOnLocation(
          location, types)
        )
      }
      if input.peek(kw::token) {
        input.parse::<kw::token>()?;

        let amount = input.parse::<IntExpr>()?;
        let token_type = input.parse::<Token>()?;
        input.parse::<kw::on>()?;
        let location = input.parse::<Location>()?;

        return Ok(Rule::CreateTokenOnLocation(
          amount, token_type, location
        ))
      }
      if input.peek(kw::precedence) {
        input.parse::<kw::precedence>()?;

        let precedence = input.parse::<Precedence>()?;
        
        if input.peek(kw::on) {
          input.parse::<kw::on>()?;

          let key_values = input.parse::<OnKeyPrec>()?;

          return Ok(Rule::CreatePrecedence(precedence, key_values))
        }

        let key_value_pairs = input.parse::<KeyValuePairs>()?;
        
        return Ok(Rule::CreatePrecedencePairs(precedence, key_value_pairs));
      }
      if input.peek(kw::pointmap) {
        input.parse::<kw::pointmap>()?;

        let pointmap = input.parse::<PointMap>()?;
        
        if input.peek(kw::on) {
          input.parse::<kw::on>()?;

          let on_key_point = input.parse::<OnKeyPoint>()?;

          return Ok(Rule::CreatePointMap(pointmap, on_key_point))
        }

        let key_value_int = input.parse::<KeyValueInt>()?;

        return Ok(Rule::CreatePointMapPairs(pointmap, key_value_int))
      }
      if input.peek(kw::combo) {
        input.parse::<kw::combo>()?;

        let combo = input.parse::<Combo>()?;
        input.parse::<Token![where]>()?;

        let filter = input.parse::<FilterExpr>()?;

        return Ok(Rule::CreateCombo(combo, filter))
      }
      if input.peek(kw::memory) {
        input.parse::<kw::memory>()?;

        let memory = input.parse::<Memory>()?;

        if input.peek(kw::on) {
          input.parse::<kw::on>()?;

          if input.peek(kw::table) {
            input.parse::<kw::table>()?;

            return Ok(Rule::CreateMemoryTable(memory))
          }

          let player_collection = input.parse::<PlayerCollection>()?;

          return Ok(Rule::CreateMemoryPlayerCollection(memory, player_collection))                
        }

        let fork = input.fork();
        if let Ok(int) = fork.parse::<IntExpr>() {
          input.advance_to(&fork);

          input.parse::<kw::on>()?;

          if input.peek(kw::table) {
            input.parse::<kw::table>()?;

            return Ok(Rule::CreateMemoryIntTable(memory, int))
          }

          let player_collection = input.parse::<PlayerCollection>()?;

          return Ok(Rule::CreateMemoryIntPlayerCollection(memory, int, player_collection))
        }

        let fork = input.fork();
        if let Ok(string) = fork.parse::<StringExpr>() {
          input.advance_to(&fork);

          input.parse::<kw::on>()?;

          if input.peek(kw::table) {
            input.parse::<kw::table>()?;

            return Ok(Rule::CreateMemoryStringTable(memory, string))
          }

          let player_collection = input.parse::<PlayerCollection>()?;

          return Ok(Rule::CreateMemoryStringPlayerCollection(memory, string, player_collection))
        }

        return Err(input.error("CreateMemry could not been parsed!"))
      }
      if input.peek(kw::flip) {
        input.parse::<kw::flip>()?;

        let cardset = input.parse::<CardSet>()?;
        input.parse::<kw::to>()?;
        let status  = input.parse::<Status>()?;

        return Ok(Rule::FlipAction(cardset, status))
      }
      if input.peek(kw::shuffle) {
        input.parse::<kw::shuffle>()?;
      
        let cardset = input.parse::<CardSet>()?;
        
        return Ok(Rule::ShuffleAction(cardset))
      }
      if input.peek(kw::set) {
        input.parse::<kw::set>()?;

        let fork = input.fork();
        if let Ok(player) = fork.parse::<PlayerExpr>() {
          input.advance_to(&fork);

          input.parse::<kw::out>()?;
          input.parse::<kw::of>()?;
          if input.peek(kw::stage) {
            input.parse::<kw::stage>()?;

            return Ok(Rule::PlayerOutOfStageAction(player))
          }
          if input.peek(kw::game) && input.peek2(kw::successful) {
            input.parse::<kw::game>()?;
            input.parse::<kw::successful>()?;

            return Ok(Rule::PlayerOutOfGameSuccAction(player))
          }
          if input.peek(kw::game) && input.peek2(kw::fail) {
            input.parse::<kw::game>()?;
            input.parse::<kw::fail>()?;

            return Ok(Rule::PlayerOutOfGameFailAction(player))
          }

          return Err(input.error("No OutAction found to parse for PlayerExpr!"))
        }
      
        let fork = input.fork();
        if let Ok(playercollection) = fork.parse::<PlayerCollection>() {
          input.advance_to(&fork);

          input.parse::<kw::out>()?;
          input.parse::<kw::of>()?;
          if input.peek(kw::stage) {
            input.parse::<kw::stage>()?;

            return Ok(Rule::PlayerCollectionOutOfStageAction(playercollection))
          }
          if input.peek(kw::game) && input.peek2(kw::successful) {
            input.parse::<kw::game>()?;
            input.parse::<kw::successful>()?;

            return Ok(Rule::PlayerCollectionOutOfGameSuccAction(playercollection))
          }
          if input.peek(kw::game) && input.peek2(kw::fail) {
            input.parse::<kw::game>()?;
            input.parse::<kw::fail>()?;

            return Ok(Rule::PlayerCollectionOutOfGameFailAction(playercollection))
          }

          return Err(input.error("No OutAction found to parse for PlayerCollection!"))
        }

        return Err(input.error("No OutAction found to parse!"))
      }
      if input.peek(kw::cycle) {
        input.parse::<kw::cycle>()?;
        input.parse::<kw::to>()?;

        let player = input.parse::<PlayerExpr>()?;

        return Ok(Rule::CycleAction(player))
      }
      if input.peek(kw::bid) {
        input.parse::<kw::bid>()?;

        let quantity = input.parse::<Quantity>()?;

        if input.peek(kw::on) {
          input.parse::<kw::on>()?;

          let memory = input.parse::<Memory>()?;

          return Ok(Rule::BidActionMemory(memory, quantity))
        }

        return Ok(Rule::BidAction(quantity))
      }
      if input.peek(kw::end) && input.peek2(kw::turn) {
        input.parse::<kw::end>()?;
        input.parse::<kw::turn>()?;

        return Ok(Rule::EndTurn)
      }
      if input.peek(kw::end) && input.peek2(kw::stage) {
        input.parse::<kw::end>()?;
        input.parse::<kw::stage>()?;

        return Ok(Rule::EndStage)
      }
      if input.peek(kw::end) && input.peek2(kw::game) {
        input.parse::<kw::end>()?;
        input.parse::<kw::game>()?;
        input.parse::<kw::with>()?;
        input.parse::<kw::winner>()?;

        let player = input.parse::<PlayerExpr>()?;

        return Ok(Rule::EndGameWithWinner(player))
      }
      if input.peek(kw::demand) {
        input.parse::<kw::demand>()?;
        let fork = input.fork();
        if let Ok(cardposition) = fork.parse::<CardPosition>() {
          input.advance_to(&fork);

          return Ok(Rule::DemandCardPositionAction(cardposition))
        }
        let fork = input.fork();
        if let Ok(string) = fork.parse::<StringExpr>() {
          input.advance_to(&fork);

          return Ok(Rule::DemandStringAction(string))
        }
        let fork = input.fork();
        if let Ok(int) = fork.parse::<IntExpr>() {
          input.advance_to(&fork);

          return Ok(Rule::DemandIntAction(int))
        }
      }

      let memory = input.parse::<Memory>()?;

      input.parse::<kw::is>()?;

      let fork = input.fork();
      if let Ok(collection) = fork.parse::<Collection>() {
          input.advance_to(&fork);

         return Ok(Rule::SetMemoryCollection(memory, collection))
      }
      
      let fork = input.fork();
      if let Ok(int) = fork.parse::<IntExpr>() {
          input.advance_to(&fork);

         return Ok(Rule::SetMemoryInt(memory, int))
      }

      let fork = input.fork();
      if let Ok(string) = fork.parse::<StringExpr>() {
          input.advance_to(&fork);

         return Ok(Rule::SetMemoryString(memory, string))
      }
      
      return Err(input.error("No Rule found to parse!"))
  }
}

impl Parse for Types {
  fn parse(input: ParseStream) -> Result<Self> {

      let mut types = Vec::new();

      let key = input.parse::<Key>()?;
      
      let content;
      parenthesized!(content in input);

      let values: Punctuated<Value, Token![,]> =
        content.parse_terminated(Value::parse, Token![,])?;

      types.push((key, values.into_iter().collect()));

      if input.peek(Token![for]) {
        input.parse::<Token![for]>()?;
        let for_types = input.parse::<Types>()?;

        types.extend_from_slice(&for_types.types);
      }

      return Ok( Types { types: types })
  }
}

impl Parse for OnKeyPrec {
  fn parse(input: ParseStream) -> Result<Self> {
      let key = input.parse::<Key>()?;

      let content;
      parenthesized!(content in input);

      let values: Punctuated<Value, Token![,]> =
        content.parse_terminated(Value::parse, Token![,])?;

      return Ok(OnKeyPrec { key, values: values.into_iter().collect()})   
  }
}

impl Parse for KeyValuePairs {
  fn parse(input: ParseStream) -> Result<Self> {
      let content;
      parenthesized!(content in input);

      let mut key_value_vec = Vec::new();
      while !content.is_empty() {
        let key = content.parse::<Key>()?;

        let in_content;
        parenthesized!(in_content in content);

        let value = in_content.parse::<Value>()?;

        key_value_vec.push((key, value));

        if content.peek(Token![,]) {
          content.parse::<Token![,]>()?;
        } else {
          break
        }
      }

      return Ok(KeyValuePairs { key_value: key_value_vec })
  }
}

impl Parse for ValueIntPair {
  fn parse(input: ParseStream) -> Result<Self> {
      let value = input.parse::<Value>()?;
      input.parse::<Token![:]>()?;
      let int = input.parse::<IntExpr>()?;

      return Ok(ValueIntPair { value, int })
  }
}

impl Parse for OnKeyPoint {
  fn parse(input: ParseStream) -> Result<Self> {
      let key = input.parse::<Key>()?;

      let content;
      parenthesized!(content in input);

      let value_int_pairs: Punctuated<ValueIntPair, Token![,]> =
        content.parse_terminated(ValueIntPair::parse, Token![,])?;

      return Ok(OnKeyPoint { key, value_int_vec: value_int_pairs.into_iter().collect() })
  }
}

impl Parse for KeyValueInt {
  fn parse(input: ParseStream) -> Result<Self> {
      let content;
      parenthesized!(content in input);

      let mut key_value_int_vec = Vec::new();
      while !content.is_empty() {
        let key = content.parse::<Key>()?;
          
        let in_content;
        parenthesized!(in_content in content);

        let value = in_content.parse::<Value>()?;
        in_content.parse::<Token![:]>()?;
        let int = in_content.parse::<IntExpr>()?;

        key_value_int_vec.push((key, value, int));

        if content.peek(Token![,]) {
          content.parse::<Token![,]>()?;
        } else {
          break
        }
      }
      
      return Ok(KeyValueInt { key_value_int_vec: key_value_int_vec })
  }
}

impl Parse for SeqStage {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::stage>()?;
      let stage = input.parse::<Stage>()?;

      input.parse::<Token![for]>()?;
      let player = input.parse::<PlayerExpr>()?;
      let endcondition = input.parse::<EndCondition>()?;

      let content;
      braced!(content in input);

      let mut flows = Vec::new();
      while !content.is_empty() {
        let flow = content.parse::<FlowComponent>()?;

        flows.push(flow);
      }

      return Ok(SeqStage {
        stage: stage,
        player: player,
        end_condition: endcondition,
        flows: flows
      })
  }
}

impl Parse for IfRule {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<Token![if]>()?;

      let content;
      parenthesized!(content in input);

      let condition = content.parse::<BoolExpr>()?;

      let content;
      braced!(content in input);

      let mut flows = Vec::new();
      while !content.is_empty() {
        let flow = content.parse::<FlowComponent>()?;

        flows.push(flow);
      }

      return Ok(IfRule { condition, flows })
  }
}

impl Parse for OptionalRule {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::optional>()?;
      
      let content;
      braced!(content in input);

      let mut flows = Vec::new();
      while !content.is_empty() {
        let flow = content.parse::<FlowComponent>()?;

        flows.push(flow);
      }

      return Ok(OptionalRule { flows })
  }
}

impl Parse for ChoiceRule {
  fn parse(input: ParseStream) -> Result<Self> {
      input.parse::<kw::choose>()?;

      let content;
      braced!(content in input);

      let options: Punctuated<FlowComponent, kw::or> =
        content.parse_terminated(FlowComponent::parse, kw::or)?;

      return Ok(ChoiceRule { options: options.into_iter().collect() })
  }
}

impl Parse for FlowComponent {
  fn parse(input: ParseStream) -> Result<Self> {
      if input.peek(kw::stage) {
        let stage = input.parse::<SeqStage>()?;

        return Ok(FlowComponent::Stage(stage))
      }
      if input.peek(Token![if]) {
        let ifrule = input.parse::<IfRule>()?;

        return Ok(FlowComponent::IfRule(ifrule))
      }
      if input.peek(kw::choose) {
        let choicerule = input.parse::<ChoiceRule>()?;

        return Ok(FlowComponent::ChoiceRule(choicerule))
      }
      if input.peek(kw::optional) {
        let optionalrule = input.parse::<OptionalRule>()?;

        return Ok(FlowComponent::OptionalRule(optionalrule))
      }

      let rule = input.parse::<Rule>()?;
      input.parse::<Token![;]>()?;

      return Ok(FlowComponent::Rule(rule))
  }
}

impl Parse for Game {
  fn parse(input: ParseStream) -> Result<Self> {
      let mut flows = Vec::new();
      while !input.is_empty() {
        let flow = input.parse::<FlowComponent>()?;

        flows.push(flow);
      }

      return Ok(Game { flows })
  }
}