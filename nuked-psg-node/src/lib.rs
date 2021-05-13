use std::cell::RefCell;

use nuked_psg_rs::*;
use neon::{context::Context, prelude::*};

fn f64_to_u8(number: f64) -> u8 {
    (number % (f64::from(u8::MAX) + 1.0)).round() as u8
}

fn f64_to_u16(number: f64) -> u16 {
    (number % (f64::from(u16::MAX) + 1.0)).round() as u16
}

fn f64_to_u32(number: f64) -> u32 {
    (number % (f64::from(u32::MAX) + 1.0)).round() as u32
}

pub struct ChipWrapper {
    pub chip: Chip
}

impl Finalize for ChipWrapper {}

type BoxedChip = JsBox<RefCell<ChipWrapper>>;

fn new(mut cx: FunctionContext) -> JsResult<BoxedChip> {
    Ok(cx.boxed(
        RefCell::new(
            ChipWrapper { chip: Chip::new()})
        )
    )
}

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);
    
    chip.chip.init();

    Ok(cx.undefined())
}

fn write(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let data = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let data = f64_to_u8(data);

    chip.chip.write(data);

    Ok(cx.undefined())
}

fn read(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    Ok(JsNumber::new(&mut cx, chip.chip.read()))
}

fn set_ic(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let ic = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let ic = f64_to_u32(ic);

    chip.chip.set_ic(ic);

    Ok(cx.undefined())
}

fn clock(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);
    
    chip.chip.clock();

    Ok(cx.undefined())
}

fn get_output(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    Ok(JsNumber::new(&mut cx, chip.chip.get_output()))
}

fn test(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);
    
    let test = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let test = f64_to_u16(test);

    chip.chip.test(test);

    Ok(cx.undefined())
}

fn generate(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    Ok(JsNumber::new(&mut cx, chip.chip.generate()))
}

fn write_buffered(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let data = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let data = f64_to_u8(data);

    chip.chip.write_buffered(data);

    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("new", new)?;
    cx.export_function("init", init)?;

    cx.export_function("write", write)?;
    cx.export_function("read", read)?;

    cx.export_function("setIC", set_ic)?;
    cx.export_function("clock", clock)?;
    cx.export_function("getOutput", get_output)?;

    cx.export_function("test", test)?;

    cx.export_function("generate", generate)?;
    cx.export_function("writeBuffered", write_buffered)?;

    Ok(())
});
