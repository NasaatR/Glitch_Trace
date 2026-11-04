//! AE adapter: stable parameter names/IDs, row-stride-aware pixels, no scene edits.
#![allow(clippy::drop_non_drop, clippy::question_mark)]
use crate::clipboard::copy_report;
use crate::core::{self, Settings};
// These two lints originate in after-effects 0.4.0 define_effect expansion.
use after_effects as ae;
use std::sync::Mutex;
use std::time::Instant;

// Names are hashed by the binding into persisted parameter IDs. Do not rename/reorder.
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params {
    KeyColor,
    Threshold,
    ShowMask,
    Blur,
    Alpha,
    Quality,
    BoxStart,
    Boxes,
    BorderColor,
    BorderWidth,
    FillColor,
    FillOpacity,
    BoxGaps,
    BoxGapSize,
    BoxEnd,
    TextStart,
    Text,
    TextColor,
    TextScale,
    TextOffset,
    TextEnd,
    MarkerStart,
    Markers,
    MarkerType,
    MarkerColor,
    MarkerSize,
    MarkerWidth,
    MarkerEnd,
    LineStart,
    Lines,
    LineColor,
    LineOpacity,
    LineWidth,
    Curvature,
    Dashed,
    DashLength,
    GapLength,
    LineEnd,
    ArrowStart,
    Arrows,
    ArrowSize,
    ArrowAngle,
    ArrowPosition,
    ArrowFilled,
    ArrowEnd,
    Diagnostics,
}
#[derive(Default)]
struct Plugin {
    last: Mutex<String>,
    error: Mutex<String>,
}
ae::define_effect!(Plugin, (), Params);
fn float(
    p: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    min: f32,
    max: f32,
    value: f64,
) -> Result<(), ae::Error> {
    p.add(
        id,
        name,
        ae::FloatSliderDef::setup(|f| {
            f.set_valid_min(min);
            f.set_valid_max(max);
            f.set_slider_min(min);
            f.set_slider_max(max);
            f.set_default(value);
            f.set_value(value);
            f.set_precision(2);
        }),
    )
}
fn check(
    p: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    value: bool,
) -> Result<(), ae::Error> {
    p.add(
        id,
        name,
        ae::CheckBoxDef::setup(|f| {
            f.set_default(value);
            f.set_value(value);
        }),
    )
}
fn color(
    p: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    c: [u8; 3],
) -> Result<(), ae::Error> {
    p.add(
        id,
        name,
        ae::ColorDef::setup(|f| {
            let c = ae::Pixel8 {
                red: c[0],
                green: c[1],
                blue: c[2],
                alpha: 255,
            };
            f.set_default(c);
            f.set_value(c);
        }),
    )
}
fn popup(
    p: &mut ae::Parameters<Params>,
    id: Params,
    name: &str,
    options: &[&str],
    value: i32,
) -> Result<(), ae::Error> {
    p.add(
        id,
        name,
        ae::PopupDef::setup(|f| {
            f.set_options(options);
            f.set_default(value);
            f.set_value(value);
        }),
    )
}
fn settings(p: &ae::Parameters<Params>, i: ae::InData) -> Result<Settings, ae::Error> {
    let f = |id| -> Result<f32, ae::Error> { Ok(p.get(id)?.as_float_slider()?.value() as f32) };
    let b = |id| -> Result<bool, ae::Error> { Ok(p.get(id)?.as_checkbox()?.value()) };
    let c = |id| -> Result<[f32; 4], ae::Error> {
        let v = p.get(id)?.as_color()?.value();
        Ok([
            v.red as f32 / 255.,
            v.green as f32 / 255.,
            v.blue as f32 / 255.,
            1.,
        ])
    };
    let pop = |id| -> Result<usize, ae::Error> {
        let n = p.get(id)?.as_popup()?.value();
        if n < 1 {
            return Err(ae::Error::InvalidParms);
        }
        Ok((n - 1) as usize)
    };
    let dx = i.downsample_x();
    let dy = i.downsample_y();
    Ok(Settings {
        key: c(Params::KeyColor)?,
        threshold: f(Params::Threshold)?,
        show_mask: b(Params::ShowMask)?,
        blur: f(Params::Blur)?,
        alpha_layer: b(Params::Alpha)?,
        quality: pop(Params::Quality)?,
        boxes: b(Params::Boxes)?,
        border: c(Params::BorderColor)?,
        border_width: f(Params::BorderWidth)?,
        fill: c(Params::FillColor)?,
        fill_opacity: f(Params::FillOpacity)? / 100.,
        gaps: b(Params::BoxGaps)?,
        gap_size: f(Params::BoxGapSize)? / 100.,
        text: b(Params::Text)?,
        text_color: c(Params::TextColor)?,
        text_scale: f(Params::TextScale)?,
        text_offset: f(Params::TextOffset)?,
        markers: b(Params::Markers)?,
        marker_type: pop(Params::MarkerType)?,
        marker_color: c(Params::MarkerColor)?,
        marker_size: f(Params::MarkerSize)?,
        marker_width: f(Params::MarkerWidth)?,
        connections: b(Params::Lines)?,
        line_color: c(Params::LineColor)?,
        line_opacity: f(Params::LineOpacity)? / 100.,
        line_width: f(Params::LineWidth)?,
        curvature: f(Params::Curvature)?,
        dashed: b(Params::Dashed)?,
        dash: f(Params::DashLength)?,
        gap: f(Params::GapLength)?,
        arrows: b(Params::Arrows)?,
        arrow_size: f(Params::ArrowSize)?,
        arrow_angle: f(Params::ArrowAngle)?,
        arrow_position: f(Params::ArrowPosition)? / 100.,
        arrow_filled: b(Params::ArrowFilled)?,
        scale: [dx.num as f32 / dx.den as f32, dy.num as f32 / dy.den as f32],
    })
}
impl AdobePluginGlobal for Plugin {
    fn params_setup(
        &self,
        p: &mut ae::Parameters<Params>,
        _: ae::InData,
        _: ae::OutData,
    ) -> Result<(), ae::Error> {
        use Params::*;
        color(p, KeyColor, "Key Color", [0, 255, 0])?;
        float(p, Threshold, "Threshold", 0., 100., 10.)?;
        check(p, ShowMask, "Show Mask", false)?;
        float(p, Blur, "Blur Strength", 0., 100., 0.)?;
        check(p, Alpha, "Alpha Layer", false)?;
        popup(
            p,
            Quality,
            "Detection Quality",
            &["Extreme", "High", "Standard", "Medium", "Low", "Draft"],
            3,
        )?;
        p.add_group(BoxStart, BoxEnd, "Bounding Box Settings", false, |p| {
            check(p, Boxes, "Enable Bounding Boxes", true)?;
            color(p, BorderColor, "Border Color", [255; 3])?;
            float(p, BorderWidth, "Border Thickness", 0., 100., 1.)?;
            color(p, FillColor, "Fill Color", [0; 3])?;
            float(p, FillOpacity, "Fill Opacity", 0., 100., 0.)?;
            check(p, BoxGaps, "Enable Gaps", false)?;
            float(p, BoxGapSize, "Gap Size", 0., 100., 20.)
        })?;
        p.add_group(TextStart, TextEnd, "Text Settings", false, |p| {
            check(p, Text, "Enable Text", true)?;
            color(p, TextColor, "Text Color", [255; 3])?;
            float(p, TextScale, "Font Scale", 0.1, 20., 1.)?;
            float(p, TextOffset, "Text Vertical Offset", -1000., 1000., 4.)
        })?;
        p.add_group(
            MarkerStart,
            MarkerEnd,
            "Center Marker Settings",
            false,
            |p| {
                check(p, Markers, "Enable Markers", true)?;
                popup(p, MarkerType, "Marker Type", &["Dot", "Plus", "Cross"], 2)?;
                color(p, MarkerColor, "Marker Color", [255; 3])?;
                float(p, MarkerSize, "Marker Size", 0., 1000., 8.)?;
                float(p, MarkerWidth, "Line Thickness", 0., 100., 1.)
            },
        )?;
        p.add_group(LineStart, LineEnd, "Connection Line Settings", false, |p| {
            check(p, Lines, "Enable Connection Lines", true)?;
            color(p, LineColor, "Line Color", [255; 3])?;
            float(p, LineOpacity, "Line Opacity", 0., 100., 100.)?;
            float(p, LineWidth, "Line Thickness", 0., 100., 1.)?;
            float(p, Curvature, "Curvature", -1., 1., 0.5)?;
            check(p, Dashed, "Enable Dashed Lines", false)?;
            float(p, DashLength, "Dash Length", 0., 1000., 10.)?;
            float(p, GapLength, "Gap Length", 0., 1000., 5.)
        })?;
        p.add_group(ArrowStart, ArrowEnd, "Arrow Settings", false, |p| {
            check(p, Arrows, "Enable Arrows", false)?;
            float(p, ArrowSize, "Arrow Size", 0., 1000., 8.)?;
            float(p, ArrowAngle, "Arrow Angle", 1., 89., 30.)?;
            float(p, ArrowPosition, "Arrow Position", 0., 100., 50.)?;
            check(p, ArrowFilled, "Filled Arrows", true)
        })?;
        p.add(
            Diagnostics,
            "Diagnostics",
            ae::ButtonDef::setup(|f| {
                f.set_label("Copy Diagnostic Report");
            }),
        )?;
        Ok(())
    }
    fn handle_command(
        &mut self,
        cmd: ae::Command,
        in_data: ae::InData,
        mut out: ae::OutData,
        p: &mut ae::Parameters<Params>,
    ) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::About=>out.set_return_msg("GlitchTrace 1.0 Preview 1\rColor detection and procedural graphics.\rMade by Nasaat\rCPU renderer; 8/16 bpc.\rNot yet verified in After Effects."),
            ae::Command::Render{in_layer,out_layer}=>{
                let started=Instant::now();
                let mut detail=String::new();
                let result=(||->Result<(),ae::Error>{
                    let s=settings(p,in_data)?;
                    let (w,h)=(in_layer.width(),in_layer.height());
                    if w!=out_layer.width()||h!=out_layer.height(){return Err(ae::Error::BadCallbackParameter)}
                    if w==0||h==0{return Ok(())}
                    let depth=in_layer.bit_depth();
                    if depth!=8&&depth!=16{return Err(ae::Error::BadCallbackParameter)}
                    if out_layer.bit_depth()!=depth{return Err(ae::Error::BadCallbackParameter)}
                    let count=w.checked_mul(h).ok_or(ae::Error::OutOfMemory)?;
                    let mut input=Vec::new();input.try_reserve_exact(count).map_err(|_|ae::Error::OutOfMemory)?;
                    for y in 0..h {
                        if y%32==0{in_data.interact().abort()?;}
                        for x in 0..w {
                            let rgba=if depth==16 {
                                let p=in_layer.as_pixel16(x,y);let k=1./32768.;
                                [p.red as f32*k,p.green as f32*k,p.blue as f32*k,p.alpha as f32*k]
                            }else{
                                let p=in_layer.as_pixel8(x,y);let k=1./255.;
                                [p.red as f32*k,p.green as f32*k,p.blue as f32*k,p.alpha as f32*k]
                            };input.push(rgba);
                        }
                    }
                    let mut abort_error=None;
                    let rendered=core::render(&input,w,h,&s,||match in_data.interact().abort(){
                        Ok(())=>true,Err(e)=>{abort_error=Some(e);false}
                    }).map_err(|e| {
                        detail=format!("src/core.rs: {e:?}; frame {w}x{h}, {depth} bpc");
                        if let Some(error)=abort_error {return error}
                        match e {core::RenderError::Allocation=>ae::Error::OutOfMemory,_=>ae::Error::BadCallbackParameter}
                    })?;
                    for y in 0..h {
                        if y%32==0{in_data.interact().abort()?;}
                        for x in 0..w {
                            let v=rendered.pixels[y*w+x];
                            if depth==16 {
                                let cv=|x:f32|(x.clamp(0.,1.)*32768.).round() as u16;
                                *out_layer.as_pixel16_mut(x,y)=ae::Pixel16{red:cv(v[0]),green:cv(v[1]),blue:cv(v[2]),alpha:cv(v[3])};
                            }else{
                                let cv=|x:f32|(x.clamp(0.,1.)*255.).round() as u8;
                                *out_layer.as_pixel8_mut(x,y)=ae::Pixel8{red:cv(v[0]),green:cv(v[1]),blue:cv(v[2]),alpha:cv(v[3])};
                            }
                        }
                    }
                    if let Ok(mut last)=self.last.lock(){*last=format!("Frame: {}x{}, {} bpc\r\nQuality: {}\r\nRegions: {}\r\nRender: {} ms\r\n",w,h,depth,s.quality+1,rendered.boxes.len(),started.elapsed().as_millis());}
                    Ok(())
                })();
                if let Err(e)=&result && let Ok(mut last)=self.error.lock(){
                    *last=format!("src/host.rs Render: {e:?}; {detail}");
                }
                result?;
            }
            ae::Command::UserChangedParam{param_index} if p.type_at(param_index)==Params::Diagnostics=>{
                let last=self.last.lock().map(|v|v.clone()).unwrap_or_else(|_|"Render report unavailable".into());
                let error=self.error.lock().map(|v|v.clone()).unwrap_or_else(|_|"Error report unavailable".into());
                let report=format!("GlitchTrace {}\r\nBuild: {}\r\nOS: {} / {}\r\nHost ID: {:?}\r\nAE effect API: {:?}\r\nRenderer: CPU / 8-16 bpc / MFR not advertised\r\n{}Last recorded error: {}\r\n\r\nReproduction: [describe the steps]\r\nExpected / actual: [describe]\r\nAE version/build: [Help > About After Effects]\r\n",env!("CARGO_PKG_VERSION"),env!("GLITCHTRACE_BUILD_SHA"),std::env::consts::OS,std::env::consts::ARCH,in_data.application_id(),in_data.version(),last,if error.is_empty(){"none"}else{&error});
                match copy_report(&report) {
                    Ok(())=>out.set_return_msg("Diagnostic report copied. Paste it into the chat or a GitHub issue.\rFor Windows/GPU details run Collect Diagnostics from the Start menu."),
                    Err(e)=>{out.set_return_msg(&format!("Could not copy diagnostic report: {e}.\rRun Collect Diagnostics from the Start menu."));out.set_out_flag(ae::OutFlags::DisplayErrorMessage,true);}
                }
            }
            _=>{}
        }
        Ok(())
    }
}
