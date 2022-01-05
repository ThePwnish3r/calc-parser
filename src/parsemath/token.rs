
/// This contains enum for list of Tokens, and handles Operator precedence rules.
#[derive(Debug, PartialEq, Clone)]
// Define the tokens we will use
pub enum Token {

	Add,

	Subtract,

	Multiply,

	Divide,
	
	//^
	Caret,
	
	LeftParen,
	
	RightParen,
	
	//for anynumber
	Num(f64),
	
	//END OF STRING 
	EOF,
}


// Order of operators as per operator precedence rules (low to high)

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines all the OperPrec levels, from lowest to highest.
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,

}




// This contains methods to retrieve operator precedence for a given arithmetic operator

impl Token {
	pub fn get_oper_prec(&self) -> OperPrec {
	    use self::OperPrec::*;
	    use self::Token::*;
	    match *self {
		Add | Subtract => AddSub,
		Multiply | Divide => MulDiv,
		Caret => Power,
    
		_ => DefaultZero,
	    }
	}

}