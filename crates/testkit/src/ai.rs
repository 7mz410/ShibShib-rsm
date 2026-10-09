//! Hand-written Illustrator editing data (the format `vectorcraft_eps`'s reader documents) and the
//! EPS and `.ai` files that carry it, for import tests. Nothing here comes from files another app
//! wrote.

/// Editing data: header comments (crop marks `0 0 width height`), `body` (setup and layers), the
/// trailer.
pub fn editing_data(width: f64, height: f64, body: &str) -> String {
    format!(
        "%!PS-Adobe-3.0\n%%Creator: VectorCraft tests\n%AI5_FileFormat 14.0\n%AI3_Cropmarks: 0 0 {width} {height}\n%%EndComments\n\
         %%BeginProlog\n%%EndProlog\n%%BeginSetup\n%%EndSetup\n{body}\n%%PageTrailer\ngsave annotatepage grestore showpage\n%%Trailer\n%%EOF\n"
    )
}

/// A layer named `name` (shown, unlocked, printing, light blue) holding `art`.
pub fn layer(name: &str, art: &str) -> String {
    format!("%AI5_BeginLayer\n1 1 1 1 0 0 1 0 79 128 255 0 50 0 Lb\n({name}) Ln\n0 A\n0 Xw\n{art}\nLB\n%AI5_EndLayer--\n")
}

/// The path of the rectangle `x0 y0 x1 y1` (art space: y up).
pub fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> String {
    format!("{x0} {y0} m\n{x0} {y1} L\n{x1} {y1} L\n{x1} {y0} L\n{x0} {y0} L\n")
}

/// `name` given to the object before it (its XML id).
pub fn named(name: &str) -> String {
    format!("%_/ArtDictionary :\n%_/XMLUID : ({}) ; (AI10_ArtUID) ,\n%_;\n%_\n", name.replace(' ', "_"))
}

/// A document of 200 × 100 pt with a bit of everything the reader reads: a layer `Art` with a
/// named group (one member hidden), a compound path, a clipping group, a radial gradient and a
/// multiplied object, and a hidden layer `Hidden` with a locked sublayer `Sub` in it.
pub fn sample() -> String {
    let red = "0 1 1 0 1 0 0 Xa\n";
    let gradient =
        "%AI5_BeginGradient: (Glow)\n(Glow) 1 2 Bd\n[\n%_0 0 0 0 1 1 1 2 1 6 50 0 Bs\n%_0 0 0 1 0 0 0 2 0.5 6 50 100 Bs\nBD\n%AI5_EndGradient\n";
    let group = format!(
        "0 Ae\nu\n{red}{}f\n{}1 Xw\n0 0 1 0 0 0 1 XA\n2 w\n{}b\n0 Xw\nU\n{}9 () XW\n",
        rect(10.0, 50.0, 40.0, 90.0),
        named("Red"),
        rect(50.0, 50.0, 80.0, 90.0),
        named("Pair")
    );
    let compound = format!("0 Ae\n*u\n{red}{}f\n{}f\n*U\n{}", rect(100.0, 50.0, 140.0, 90.0), rect(110.0, 60.0, 130.0, 80.0), named("Ring"));
    let clip =
        format!("0 Ae\nq\n0.5 g\n{}f\n{}h\nW\nn\nQ\n{}9 () XW\n", rect(150.0, 50.0, 190.0, 90.0), rect(160.0, 60.0, 180.0, 80.0), named("Window"));
    let shaded = format!(
        "{}Bb\n1 (Glow) 0 0 0 1 1 0 0 1 0 0 1 Bg\n20 0 0 -20 30 25 Bm\nf\n0 BB\n{}2 0.5 0 0 0 Xy\n{}f\n0 1 0 0 0 Xy\n",
        rect(10.0, 5.0, 50.0, 45.0),
        named("Shaded"),
        rect(60.0, 5.0, 90.0, 45.0)
    );
    let sub = "%AI5_BeginLayer\n1 1 0 1 0 0 1 2 79 255 79 0 50 0 Lb\n(Sub) Ln\n0 g\n10 10 m\n30 30 L\nS\nLB\n%AI5_EndLayer--\n";
    let hidden = format!("%AI5_BeginLayer\n0 1 1 1 0 0 0 1 255 79 79 0 50 0 Lb\n(Hidden) Ln\n{sub}LB\n%AI5_EndLayer--\n");
    format!("{gradient}{}{hidden}", layer("Art", &format!("{group}{compound}{clip}{shaded}")))
}

/// A layer `Images` with a 2 × 1 RGB image with an alpha channel at (100, 40) in art space, 20 pt a
/// pixel.
pub fn image_layer() -> Vec<u8> {
    let mut s = b"%AI5_BeginLayer\n1 1 1 1 0 0 1 0 79 128 255 0 50 0 Lb\n(Images) Ln\n%AI5_File:\n%AI5_BeginRaster\n() 1 XG\n/DeviceRGB XN\n\
        [ 20 0 0 20 100 40 ] 2 1 0 Xh\n[ 20 0 0 20 100 40 ] 0 0 2 1 2 1 8 3 1 17 1 0 4 4 0 0\n%%BeginData: 10\rXI\n"
        .to_vec();
    s.extend_from_slice(&[255, 0, 0, 0, 0, 255, 255, 128]);
    s.extend_from_slice(b"%%EndData\r\nXH\r\n%AI5_EndRaster\r\nN\r\nLB\r\n%AI5_EndLayer--\r\n");
    s
}

/// [`sample`] with [`image_layer`] on top, as the editing data of a 200 × 100 pt document.
pub fn sample_data() -> Vec<u8> {
    let mut data = editing_data(200.0, 100.0, &sample()).into_bytes();
    let at = data.windows(14).position(|w| w == b"%%PageTrailer\n").unwrap();
    data.splice(at..at, image_layer());
    data
}

/// An Illustrator EPS: a page drawing one line (what readers without the editing data see), and
/// the editing data `data` after it, not compressed.
pub fn eps(data: &[u8]) -> Vec<u8> {
    let mut out = b"%!PS-Adobe-3.0 EPSF-3.0\n%%Creator: VectorCraft tests\n%%BoundingBox: 0 0 200 100\n%%EndComments\n\
        0 0 moveto 200 100 lineto stroke\nshowpage\n%%EOF\n%AI9_PrivateDataBegin\n"
        .to_vec();
    out.extend_from_slice(data);
    out.extend_from_slice(b"\n%AI9_PrivateDataEnd\n");
    out
}

/// An Illustrator `.ai` file: a PDF page drawing one line, carrying `data` (the private data
/// streams' contents, such as `%AI12_CompressedData` and zlib data) in two streams.
pub fn ai(data: &[u8]) -> Vec<u8> {
    let (first, second) = data.split_at(data.len() / 2);
    let stream = |part: &[u8]| {
        let mut s = format!("<< /Length {} >>\nstream\n", part.len()).into_bytes();
        s.extend_from_slice(part);
        s.extend_from_slice(b"\nendstream");
        s
    };
    let objects: Vec<Vec<u8>> = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Contents 4 0 R /Resources << >> \
          /PieceInfo << /Illustrator << /Private << /AIPrivateData1 5 0 R /AIPrivateData2 6 0 R /NumBlock 2 >> >> >> >>"
            .to_vec(),
        stream(b"0 0 m 200 100 l S"),
        stream(first),
        stream(second),
    ];
    let mut out = b"%PDF-1.5\n".to_vec();
    let mut offsets = vec![];
    for (i, o) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend(format!("{} 0 obj\n", i + 1).bytes());
        out.extend_from_slice(o);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).bytes());
    for off in offsets {
        out.extend(format!("{off:010} 00000 n \n").bytes());
    }
    out.extend(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objects.len() + 1).bytes());
    out
}
