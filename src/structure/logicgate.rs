

///逻辑门
pub trait LogicGate {
    fn get_result(&self) -> bool;
}



///与非门 全为true则为false
/// # Examples
/// ```
/// use algori::structure::Nand;
/// use algori::structure::LogicGate;
/// let a = Nand{input1: true, input2: false};
/// assert_eq!(a.get_result(),true)
/// ```
#[derive(PartialEq)]
pub struct Nand {
    pub input1: bool,
    pub input2: bool,
}


impl LogicGate for Nand {

    fn get_result(&self) -> bool {
	if self.input1 == true{
	    if self.input2 == true {
		return false;
	    }
	}
	true
    }
}

///非门 反转真值表
///非门由一个输入一致的与非门组成
/// # Examples
/// ```
/// use algori::structure::Not;
/// use algori::structure::LogicGate;
/// let a = Not{input: true};
/// let b = Not{input: false};
/// assert_eq!(a.get_result(),false);
/// assert_eq!(b.get_result(),true);
/// ```
pub struct Not {
    pub input: bool,
}


impl LogicGate for Not {
    fn get_result(&self) -> bool {
	let a = Nand{input1: self.input, input2: self.input};
	a.get_result()
    }
}
///或门 有true则为true
///或门由三个与非门组成
/// # Examples
/// ```
/// use algori::structure::Or;
/// use algori::structure::LogicGate;
/// let a = Or{input1: true,input2: false};
/// let b = Or{input1: false,input2: false};
/// assert_eq!(a.get_result(),true);
/// assert_eq!(b.get_result(),false);
/// ```
pub struct Or {
    pub input1: bool,
    pub input2: bool,
}

impl LogicGate for Or {
    fn get_result(&self) -> bool {
	let a = Nand{input1: self.input1, input2: self.input1};
	let b = Nand{input1: self.input2, input2: self.input2};
	let c = Nand{input1: a.get_result(),input2: b.get_result()};
	c.get_result()
    }
}

///或非门 有true则为false
///或非门由四个与非门组成
/// # Examples
/// ```
/// use algori::structure::Nor;
/// use algori::structure::LogicGate;
/// let a = Nor{input1: true,input2: false};
/// let b = Nor{input1: false,input2: false};
/// assert_eq!(a.get_result(),false);
/// assert_eq!(b.get_result(),true);
/// ```
pub struct Nor{
    pub input1: bool,
    pub input2: bool,
}

impl LogicGate for Nor {
    fn get_result(&self) -> bool {
	let a = Nand{input1: self.input1, input2: self.input1};
	let b = Nand{input1: self.input2, input2: self.input2};
	let c = Nand{input1: a.get_result(),input2: b.get_result()};
	let d = Nand{input1: c.get_result(),input2: c.get_result()};
	d.get_result()
    }
}
///与门 全为true则为true
///与门由两个与非门组成
/// # Examples
/// ```
/// use algori::structure::And;
/// use algori::structure::LogicGate;
/// let a = And{input1: true,input2: false};
/// let b = And{input1: true,input2: true};
/// assert_eq!(a.get_result(),false);
/// assert_eq!(b.get_result(),true);
/// ```
pub struct And {
    pub input1: bool,
    pub input2: bool,
}

impl LogicGate for And {
    fn get_result(&self) -> bool {
	let a = Nand{input1: self.input1, input2: self.input2};
	let b = Nand{input1: a.get_result(), input2: a.get_result()};
	b.get_result()
    }
}
///高电平 输出true
/// # Examples
/// ```
/// use algori::structure::HighLevel;
/// use algori::structure::LogicGate;
/// let a = HighLevel{};
/// assert_eq!(a.get_result(),true);
/// ```
pub struct HighLevel{
}
impl LogicGate for HighLevel {
    fn get_result(&self) -> bool {
	true
    }
}

///低电平 输出false
/// # Examples
/// ```
/// use algori::structure::LowLevel;
/// use algori::structure::LogicGate;
/// let a = LowLevel{};
/// assert_eq!(a.get_result(),false);
/// ```
pub struct LowLevel{
}
impl LogicGate for LowLevel {
    fn get_result(&self) -> bool {
	false
    }
}

///异或门 输入不一样true
/// # Examples
/// ```
/// use algori::structure::Xor;
/// use algori::structure::LogicGate;
/// let a = Xor{input1:true,input2:false};
/// let b = Xor{input1:true,input2:true};
/// let c = Xor{input1:false,input2:false};
/// assert_eq!(a.get_result(),true);
/// assert_eq!(b.get_result(),false);
/// assert_eq!(c.get_result(),false);
/// ```
pub struct Xor{
    pub input1: bool,
    pub input2: bool,
}
impl LogicGate for Xor{
    fn get_result(&self) ->bool {
	let a:And = And{input1:self.input1,input2:self.input2};
	let b:Nor = Nor{input1:self.input1,input2:self.input2};
	let c:Nor = Nor{input1:a.get_result(),input2:b.get_result()};
	c.get_result()
    }
}


///三路或门
/// # Examples
/// ```
/// use algori::structure::ThreeOr;
/// use algori::structure::LogicGate;
/// let a = ThreeOr{input1:true,input2:false,input3:false};
/// let b = ThreeOr{input1:false,input2:true,input3:false};
/// let c = ThreeOr{input1:false,input2:false,input3:true};
/// let d = ThreeOr{input1:false,input2:false,input3:false};
/// assert_eq!(a.get_result(),true);
/// assert_eq!(b.get_result(),true);
/// assert_eq!(c.get_result(),true);
/// assert_eq!(d.get_result(),false);
/// ```
pub struct ThreeOr{
    pub input1: bool,
    pub input2: bool,
    pub input3: bool,
}

impl LogicGate for ThreeOr{
    fn get_result(&self) ->bool {
	let a:Or = Or{input1:self.input1,input2:self.input2};
	let b:Or = Or{input1:self.input2,input2:self.input3};
	let c:Or = Or{input1:a.get_result(),input2:b.get_result()};
	c.get_result()
    }
}


///三路与门
/// # Examples
/// ```
/// use algori::structure::ThreeAnd;
/// use algori::structure::LogicGate;
/// let a = ThreeAnd{input1:true,input2:false,input3:false};
/// let b = ThreeAnd{input1:false,input2:true,input3:false};
/// let c = ThreeAnd{input1:false,input2:false,input3:true};
/// let d = ThreeAnd{input1:false,input2:false,input3:false};
/// let e = ThreeAnd{input1:true,input2:true,input3:true};
/// assert_eq!(a.get_result(),false);
/// assert_eq!(b.get_result(),false);
/// assert_eq!(c.get_result(),false);
/// assert_eq!(d.get_result(),false);
/// assert_eq!(e.get_result(),true);
/// ```
pub struct ThreeAnd{
    pub input1: bool,
    pub input2: bool,
    pub input3: bool,
}

impl LogicGate for ThreeAnd{
    fn get_result(&self) ->bool {
	let a:And = And{input1:self.input1,input2:self.input2};
	let b:And = And{input1:self.input2,input2:self.input3};
	let c:And = And{input1:a.get_result(),input2:b.get_result()};
	c.get_result()
    }
}
///同或门
///相同则为true
/// # Examples
/// ```
/// use algori::structure::Xnor;
/// use algori::structure::LogicGate;
/// let a = Xnor{input1:true,input2:false};
/// let b = Xnor{input1:true,input2:true};
/// let c = Xnor{input1:false,input2:false};
/// assert_eq!(a.get_result(),false);
/// assert_eq!(b.get_result(),true);
/// assert_eq!(c.get_result(),true);
/// ```
pub struct Xnor{
    pub input1: bool,
    pub input2: bool,
}

impl LogicGate for Xnor{
    fn get_result(&self) ->bool {
	let a:Xor = Xor{input1:self.input1,input2:self.input2};
	let b:Not = Not{input: a.get_result()};
	b.get_result()
    }
}

use std::time::Duration;
///延迟线
///Examples
///```
/// use algori::structure::DelayLine;
/// let a = DelayLine{delay: 20,input: true};
/// 
/// a.get_result();
///```

pub struct DelayLine {
    /// 延迟时间，单位为毫秒
    pub delay: u64,
    pub input: bool,
}

impl DelayLine {
    pub fn get_result(&self) -> bool {
        std::thread::sleep(Duration::from_millis(self.delay));
        self.input
    }
}

///半加器
///输出一个包含低位结果与进位的元组
/// Examples
///```
///use algori::structure::HalfAdder;
///let a:HalfAdder = HalfAdder{input1:true,input2:false};
///assert_eq!(a.get_result(),(true,false));
///let a:HalfAdder = HalfAdder{input1:true,input2:true};
///assert_eq!(a.get_result(),(false,true));
pub struct HalfAdder {
    pub input1: bool,
    pub input2: bool,
}



impl HalfAdder{
    ///返回(sum,carry)
    pub fn get_result(&self) -> (bool,bool) {
	let a:Xor = Xor{input1: self.input1,input2:self.input2};
	let b:And = And{input1: self.input1,input2:self.input2};
	(a.get_result(),b.get_result())
    }
}
