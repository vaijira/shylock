use crate::scraper::BASE_BOE_URL;
use scraper::{Html, Selector};
use shylock_data::concepts::BoeConcept;
use std::collections::HashMap;

const RESULTS_PER_PAGE: u32 = 500;

fn parse_html_table(
    page: &str,
    data_selector: &Selector,
) -> Result<HashMap<BoeConcept, String>, Box<dyn std::error::Error>> {
    let mut result: HashMap<BoeConcept, String> = HashMap::new();

    let doc = Html::parse_document(page);
    let data = doc
        .select(data_selector)
        .next()
        .ok_or("error parsing table, no container of table found")?;
    let tr_selector = Selector::parse("tr").expect("tr selector creation failed");
    let th_selector = Selector::parse("th").expect("th selector creation failed");
    let td_selector = Selector::parse("td").expect("td selector creation failed");

    for tr in data.select(&tr_selector) {
        let th = tr
            .select(&th_selector)
            .next()
            .ok_or("not found th")?
            .text()
            .collect::<String>();
        let td = tr
            .select(&td_selector)
            .next()
            .ok_or("not found td")?
            .text()
            .collect::<String>();

        result.insert(th.trim().parse::<BoeConcept>()?, td.trim().to_owned());
    }

    Ok(result)
}

pub(crate) fn parse_management_auction_page(
    page: &str,
) -> Result<HashMap<BoeConcept, String>, Box<dyn std::error::Error>> {
    parse_html_table(
        page,
        &Selector::parse(r#"div[id=idBloqueDatos2]"#)
            .expect("div[id=idBloqueDatos2] selector creation failed"),
    )
}

pub(crate) fn parse_asset_auction_page(
    page: &str,
) -> Result<HashMap<BoeConcept, String>, Box<dyn std::error::Error>> {
    let h4_selector = &Selector::parse("h4").expect("h4 selector creation failed");
    let data_selector = &Selector::parse(r#"div[id=idBloqueLote1]"#)
        .expect("div[id=idBloqueLote1 selector creation failed");

    let mut result = parse_html_table(page, data_selector)?;

    let doc = Html::parse_document(page);
    let data = doc.select(data_selector).next().ok_or("no div found")?;
    let header = data
        .select(h4_selector)
        .next()
        .ok_or("no header h4 found")?;

    result.insert(
        BoeConcept::Header,
        header.text().collect::<String>().trim().to_uppercase(),
    );

    Ok(result)
}

pub(crate) fn parse_lot_auction_page_links(
    page: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    let doc = Html::parse_document(page);

    let ul_selector =
        Selector::parse("ul.navlistver").expect("ul.navlistver selector creation failed");
    let a_selector = Selector::parse("a").expect("a selector creation failed");
    let ul = doc
        .select(&ul_selector)
        .next()
        .ok_or("not ul element found")?;

    for lot_anchor in ul.select(&a_selector) {
        if let Some(href) = lot_anchor.value().attr("href") {
            result.push(BASE_BOE_URL.to_owned() + href);
        }
    }

    Ok(result)
}

pub(crate) fn parse_lot_auction_page(
    page: &str,
    i: usize,
) -> Result<HashMap<BoeConcept, String>, Box<dyn std::error::Error>> {
    let h4_selector = &Selector::parse("h4").expect("h4 selector creation failed");
    let div_str_selector = format!(r#"div[id=idBloqueLote{}]"#, i);
    let data_selector = &Selector::parse(&div_str_selector).expect("div[id=idBloqueLoteX] failed");

    let mut result = parse_html_table(page, data_selector)?;

    let doc = Html::parse_document(page);
    let data = doc
        .select(data_selector)
        .next()
        .ok_or("no div[id=idBloqueLoteX] field found")?;
    let header = data.select(h4_selector).next().ok_or("no h4 field found")?;

    result.insert(
        BoeConcept::Header,
        header.text().collect::<String>().trim().to_uppercase(),
    );

    Ok(result)
}

pub(crate) fn parse_main_auction_links(
    page: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let doc = Html::parse_document(page);
    let links_selector =
        Selector::parse("ul.navlist").expect("ul.navlist selector creation failed");
    let data = doc
        .select(&links_selector)
        .next()
        .ok_or("unable to select ul.navlist links")?;
    let link_selector = Selector::parse("a").expect("a selector creation failed");

    let mut iterator = data.select(&link_selector);
    iterator.next();

    let management_link = iterator
        .next()
        .ok_or("error looking for a link")?
        .value()
        .attr("href")
        .ok_or("error looking for href attribute")?;

    let asset_link = iterator
        .next()
        .ok_or("error looking for a link")?
        .value()
        .attr("href")
        .ok_or("error looking for href attribute")?;

    Ok((
        BASE_BOE_URL.to_owned() + management_link,
        BASE_BOE_URL.to_owned() + asset_link,
    ))
}

pub(crate) fn parse_main_auction_page(
    page: &str,
) -> Result<HashMap<BoeConcept, String>, Box<dyn std::error::Error>> {
    parse_html_table(
        page,
        &Selector::parse(r#"div[id=idBloqueDatos1]"#)
            .expect("div[id=idBloqueDatos1] selector creation failed"),
    )
}

pub(crate) fn parse_extra_pages(main_page: &str) -> Vec<String> {
    let mut result = Vec::new();
    let doc = Html::parse_document(main_page);
    let pages_number_p =
        Selector::parse("div.paginar").expect("div.paginar selector creation failed");

    let pages = if let Some(paragraph) = doc.select(&pages_number_p).next() {
        let text = paragraph.text().collect::<String>();
        let words = text.trim().split(' ');

        let results = words
            .last()
            .expect("number")
            .replace('.', "")
            .parse::<u32>()
            .unwrap();
        (results / RESULTS_PER_PAGE) + 1
    } else {
        panic!("Unable to determine number of auctions pages");
    };

    let pages_div = Selector::parse("div.paginar2").expect("div.paginar2 selector creation failed");

    if let Some(div) = doc.select(&pages_div).next() {
        let anchors_selector = Selector::parse("a").expect("a selector creation failed");
        if let Some(page_anchor) = div.select(&anchors_selector).next() {
            if let Some(href_tmp) = page_anchor.value().attr("href") {
                let href_template = href_tmp
                    .chars()
                    .take_while(|c| *c != '-')
                    .collect::<String>();
                for page in 1..pages + 1 {
                    let href = href_template.to_owned()
                        + "-"
                        + &(page * RESULTS_PER_PAGE).to_string()
                        + "-"
                        + &RESULTS_PER_PAGE.to_string();
                    result.push(BASE_BOE_URL.to_owned() + &href);
                }
            }
        }
    }

    result.pop();

    result
}

pub(crate) fn parse_result_page(page: &str) -> Vec<String> {
    let mut result = Vec::new();

    let doc = Html::parse_document(page);

    let auction_anchors = Selector::parse("a.resultado-busqueda-link-otro")
        .expect("a.resultado-busueda-link-otro selector creation failed");

    for auction_anchor in doc.select(&auction_anchors) {
        if let Some(href) = auction_anchor.value().attr("href") {
            result.push(BASE_BOE_URL.to_owned() + href);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lot_auction_page_links_test() {
        const INPUT: &str = r#"<div id="cont-tabs">
      <div id="tabsver">
        <ul class="navlistver">
          <li>
            <a id="idTabLote1" href="./detalleSubasta.php?idSub=SUB-JA-2020-158475&amp;ver=3&amp;idLote=1&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;numPagBus=#cont-tabs" title="FINCA REGISTRAL N&#xBA; 29.047 DEL REGISTRO DE LA PROPIEDAD N&#xBA;1 DE LOGRO&#xD1;O"><span class="pc">Lote </span>1</a>
          </li>
          <li>
            <a id="idTabLote2" href="./detalleSubasta.php?idSub=SUB-JA-2020-158475&amp;ver=3&amp;idLote=2&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;numPagBus=#cont-tabs" title="FINCA REGISTRAL N&#xBA;29.023-45 DEL REGISTRO DE LA PROPIEDAD N&#xBA;1 DE LOGRO&#xD1;O" class="current"><span class="pc">Lote </span>2</a>
          </li>
        </ul>
      </div>
      </div>
      "#;
        let pages = parse_lot_auction_page_links(INPUT).unwrap();
        assert_eq!(2, pages.len());
        assert_eq!(&"https://subastas.boe.es/./detalleSubasta.php?idSub=SUB-JA-2020-158475&ver=3&idLote=1&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&numPagBus=#cont-tabs",
                 pages.get(0).unwrap());
        assert_eq!(&"https://subastas.boe.es/./detalleSubasta.php?idSub=SUB-JA-2020-158475&ver=3&idLote=2&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&numPagBus=#cont-tabs",
                 pages.get(1).unwrap());
    }
    #[test]
    fn parse_multiple_lot_auction_page_test() {
        const _INPUT: &str = r#"<div id="idBloqueDatos3">
      <div>
        <div class="bloque" id="idBloqueLote1">
          <div>
            <div class="caja">SUBASTA DE 2 VEH&#xCD;CULOS</div>
          </div>
          <div>
            <h3>Datos de los bienes subastados</h3>
            <div>
              <h4>Bien 1 - Bien mueble (Otros bienes y derechos)</h4>
              <table>
                <tr>
                  <th>Descripci&#xF3;n</th>
                  <td>TRACTOR DE CAMI&#xD3;N, marca SCANIA R480 LA 4X2 MEB
    Matr&#xED;cula 9368HYC
    Fecha 1&#xAA; matriculaci&#xF3;n 06/01/2010
    Fecha rematriculaci&#xF3;n 30/06/2014
    Bastidor XLER4X20095230715
    Contrase&#xF1;a Homologaci&#xF3;n C-2417
    Kil&#xF3;metros Seg&#xFA;n ITV a 25/02/2020 ten&#xED;a 1055479.
    Carburante Di&#xE9;sel
    Color Gris
    Cilindrada 12740.0 CC
    Potencia 353 kW / 47.57 cvf</td>
                </tr>
                <tr>
                  <th>Dep&#xF3;sito</th>
                  <td>Ubicaci&#xF3;n de los bienes objeto de subasta: Avenida de los Cordeleros n&#xBA; 17 y en la planta de
    hormig&#xF3;n Fernando Rico, Segovia.</td>
                </tr>
                <tr>
                  <th>Visitable</th>
                  <td>S&#xED; (Es posible visitar los veh&#xED;culos por las personas interesadas pudiendo
    asistirles los peritos o t&#xE9;cnicos que crean oportunos, sin posibilidad de prueba de funcionamiento,
    y debiendo concretar una cita previamente a trav&#xE9;s de esta oficina.)</td>
                </tr>
                <tr>
                  <th>Cargas</th>
                  <td>NO CONSTAN</td>
                </tr>
                <tr>
                  <th>Informaci&#xF3;n adicional</th>
                  <td>Chapa y pintura El parachoques est&#xE1; rallado.
    Ruedas en mal estado.                                    Interior Ocularmente se observa que el asiento y el salpicadero tienen algunos da&#xF1;os.
    Motor No se puso en marcha el veh&#xED;culo por lo que se desconoce el estado y funcionamiento del motor, cuadros y luces.
    Documentaci&#xF3;n y llaves Existe documentaci&#xF3;n pero no existen llaves
    ITV La &#xFA;ltima inspecci&#xF3;n t&#xE9;cnica caduc&#xF3; el 25/08/2020
    </td>
                </tr>
              </table>
            </div>
            <div>
              <h4>Bien 2 - Bien mueble (Otros bienes y derechos)</h4>
              <table>
                <tr>
                  <th>Descripci&#xF3;n</th>
                  <td>Semirremolque Caja
    Marca LECITRAILER
    Modelo 3E20
    Matr&#xED;cula R1407BCF
    Fecha 1&#xAA; matriculaci&#xF3;n 19/02/2008
    Bastidor VV1P3AAWASN135717
    Contrase&#xF1;a Homologaci&#xF3;n D-2440
    Color Azul
    Cilindrada 12740.0 CC
    Otros TARA 7720 y MMA 35000</td>
                </tr>
                <tr>
                  <th>Dep&#xF3;sito</th>
                  <td>Ubicaci&#xF3;n de los bienes objeto de subasta: Avenida de los Cordeleros n&#xBA; 17 y en la planta de
    hormig&#xF3;n Fernando Rico, Segovia.</td>
                </tr>
                <tr>
                  <th>Visitable</th>
                  <td>S&#xED; (Es posible visitar los veh&#xED;culos por las personas interesadas pudiendo
    asistirles los peritos o t&#xE9;cnicos que crean oportunos, sin posibilidad de prueba de funcionamiento,
    y debiendo concretar una cita previamente a trav&#xE9;s de esta oficina.)</td>
                </tr>
                <tr>
                  <th>Cargas</th>
                  <td>NO CONSTAN</td>
                </tr>
                <tr>
                  <th>Informaci&#xF3;n adicional</th>
                  <td>Lunas e intermitencias Intermitente izquierdo roto
    Ruedas en mal estado.
    Documentaci&#xF3;n y llaves No existe documentaci&#xF3;n ni llaves
    ITV La &#xFA;ltima inspecci&#xF3;n t&#xE9;cnica caduc&#xF3; el 20/02/2020.</td>
                </tr>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>"#;
    }
    #[test]
    fn parse_lot_auction_page_test() {
        const INPUT: &str = r#" <div class="bloque" id="idBloqueLote2">
        <div>
          <div class="tablet movil">
            <h3>Lote 2</h3>
          </div>
          <div class="caja">FINCA REGISTRAL Nº29.023-45 DEL REGISTRO DE LA PROPIEDAD Nº1 DE LOGROÑO</div>
        </div>
        <div>
          <h3>Datos relacionados con la subasta del lote 2</h3>
          <table>
            <tr>
              <th>Valor Subasta</th>
              <td>15.100,00 €</td>
            </tr>
            <tr>
              <th>Importe del dep&#xF3;sito</th>
              <td>755,00 €</td>
            </tr>
            <tr>
              <th>Puja m&#xED;nima</th>
              <td>Sin puja mínima</td>
            </tr>
            <tr>
              <th>Tramos entre pujas</th>
              <td>302,00 €</td>
            </tr>
          </table>
        </div>
        <div>
          <h3>Datos del bien subastado</h3>
          <div>
            <h4>Bien 1 - Inmueble (Garaje)</h4>
            <table>
              <tr>
                <th>Descripción</th>
                <td>GARAJE SITO EN LOGROÑO</td>
              </tr>
              <tr>
                <th>Direcci&#xF3;n</th>
                <td>AVENIDA MANUEL DE FALLA Nº51 SOTANA Nº1</td>
              </tr>
              <tr>
                <th>Código Postal</th>
                <td>26007</td>
              </tr>
              <tr>
                <th>Localidad</th>
                <td>LOGROÑO</td>
              </tr>
              <tr>
                <th>Provincia</th>
                <td>La Rioja</td>
              </tr>
              <tr>
                <th>Situación posesoria</th>
                <td>No consta</td>
              </tr>
              <tr>
                <th>Visitable</th>
                <td>No consta</td>
              </tr>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>"#;

        let lot: HashMap<BoeConcept, String> = [
            (BoeConcept::AuctionValue, String::from("15.100,00 €")),
            (BoeConcept::DepositAmount, String::from("755,00 €")),
            (BoeConcept::MinimumBid, String::from("Sin puja mínima")),
            (BoeConcept::BidStep, String::from("302,00 €")),
            (
                BoeConcept::Header,
                String::from("BIEN 1 - INMUEBLE (GARAJE)"),
            ),
            (
                BoeConcept::Description,
                String::from("GARAJE SITO EN LOGROÑO"),
            ),
            (
                BoeConcept::Address,
                String::from("AVENIDA MANUEL DE FALLA Nº51 SOTANA Nº1"),
            ),
            (BoeConcept::PostalCode, String::from("26007")),
            (BoeConcept::City, String::from("LOGROÑO")),
            (BoeConcept::Province, String::from("La Rioja")),
            (BoeConcept::OwnerStatus, String::from("No consta")),
            (BoeConcept::Visitable, String::from("No consta")),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(lot, parse_lot_auction_page(INPUT, 2).unwrap());
    }

    #[test]
    fn parse_main_auction_links_test() {
        const INPUT: &str = r#"<div id="tabs">
        <input type="checkbox" class="desplegable" id="dropDownFiltro" value="" name="dropDownFiltro"/>
        <label class="selected" for="dropDownFiltro" data-toggle="dropdown">Informaci&#xF3;n general</label>
        <ul class="navlist">
          <li>
            <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149474&amp;ver=1&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;idLote=1&amp;numPagBus=" class="current">Informaci&#xF3;n general</a>
          </li>
          <li>
            <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149474&amp;ver=2&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;idLote=1&amp;numPagBus=">Autoridad gestora</a>
          </li>
          <li>
            <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149474&amp;ver=3&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;idLote=1&amp;numPagBus=">Lotes</a>
          </li>
          <li>
            <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149474&amp;ver=4&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;idLote=1&amp;numPagBus=">Relacionados</a>
          </li>
          <li>
            <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149474&amp;ver=5&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&amp;idLote=1&amp;numPagBus=">Pujas</a>
          </li>
        </ul>
      </div>"#;
        let (link1, link2) = parse_main_auction_links(INPUT).unwrap();

        assert_eq!("https://subastas.boe.es/./detalleSubasta.php?idSub=SUB-JA-2020-149474&ver=2&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&idLote=1&numPagBus=",
                 link1);
        assert_eq!("https://subastas.boe.es/./detalleSubasta.php?idSub=SUB-JA-2020-149474&ver=3&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,&idLote=1&numPagBus=",
                 link2);
    }
    #[test]
    fn parse_extra_pages_test() {
        const INPUT: &str = r##"<body>
        <div class="paginar">
        <p>Resultados 1 a 500 de 1.572</p>
      </div>
      <div class="paginar2">
        <ul>
          <li>
            <span class="fuera">Está usted en la página de resultados número </span>
            <span class="current">1</span>
          </li>
          <li>
            <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-500-500">2</a>
          </li>
          <li>
            <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-1000-500">3</a>
          </li>
          <li>
            <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-1500-500">4</a>
          </li>
          <li>
            <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-500-500"><abbr title="Página">Pág.</abbr> siguiente</a>
          </li>
        </ul>
      </div>
    <div class="paginar">
    <p class="linkSubir">
      <a href="#top">subir</a>
    </p>
  </div>
  <div class="paginar2">
    <ul>
      <li>
        <span class="fuera">Está usted en la página de resultados número </span>
        <span class="current">1</span>
      </li>
      <li>
        <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-500-500">2</a>
      </li>
      <li>
        <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-1000-500">3</a>
      </li>
      <li>
        <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-1500-500">4</a>
      </li>
      <li>
        <a href="subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-500-500"><abbr title="Página">Pág.</abbr> siguiente</a>
      </li>
    </ul>
  </div></body>"##;
        let pages = parse_extra_pages(INPUT);
        assert_eq!(3, pages.len());
        assert_eq!(&"https://subastas.boe.es/subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-500-500",
                   pages.get(0).unwrap());
        assert_eq!(&"https://subastas.boe.es/subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-1000-500",
                   pages.get(1).unwrap());
        assert_eq!(&"https://subastas.boe.es/subastas_ava.php?accion=Mas&id_busqueda=_YjU3T1REVDZIbFlRRXkwMGhrRC9PZlorZ3RmRDVXL210ZXN4QU1aVWxpL2RjNDVLQldmR2tETFZNWnpmOUcxdXE4a2NBWnhtZ1NHWGxiVGxUdG1mQm1yKzArdk1nOW1IWEs0WTU4VTJnV01iZ1huaEVhSFVqbHplTkp4Nm5DV1RtMFVocDNiYThvbWZ4a1FYcm9lWDJCNFM4bUVHUnRKVWxDdmF5bXZSUVNFY3lGTytyQTlKMFBLUjNVejdVbUU1aW95ZTV3Q2RRbW5kOERKNkpZMDkwY3VkcVhoa3FhWERudXpuc0tSdXVaOTlZNTVwU1F6aWYrbmpWSmVBZERJUg,,-1500-500",
                   pages.get(2).unwrap());
    }

    #[test]
    fn parse_asset_auction_test() {
        const INPUT: &str = r#"<div class="bloque" id="idBloqueLote1">
    <div>
      <div class="caja">FINCA URBANA, SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM.90, PLANTA BAJA-1º. FINCA  NUM. 17228</div>
    </div>
    <div>
      <h3>Datos del bien subastado</h3>
      <div>
        <h4>Bien 1 - Inmueble (Vivienda)</h4>
        <table>
          <tr>
            <th>Descripción</th>
            <td>FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM.90, BAJO-1º</td>
          </tr>
          <tr>
            <th>Referencia catastral</th>
            <td>
              <a href="consultaDnprc.php?rc=4110202UM5141A0003HH&amp;idSub=SUB-JA-2020-149494" target="_blank" title="Abre datos catastrales en nueva ventana" onclick="return confirm('El Portal de Subastas se va conectar a los servicios web de la Dirección General del Catastro y mostrará la información en una nueva ventana');">4110202UM5141A0003HH</a>
            </td>
          </tr>
          <tr>
            <th>Dirección</th>
            <td>CALLE MARIANO DE LOS COBOS 90</td>
          </tr>
          <tr>
            <th>Código Postal</th>
            <td>47014</td>
          </tr>
          <tr>
            <th>Localidad</th>
            <td>VALLADOLID</td>
          </tr>
          <tr>
            <th>Provincia</th>
            <td>Valladolid</td>
          </tr>
          <tr>
            <th>Vivienda habitual</th>
            <td>Sí</td>
          </tr>
          <tr>
            <th>Situación posesoria</th>
            <td>No consta</td>
          </tr>
          <tr>
            <th>Visitable</th>
            <td>No consta</td>
          </tr>
          <tr>
            <th>Inscripción registral</th>
            <td>CONSTA EN EL EDICTO</td>
          </tr>
        </table>
      </div>
    </div>
  </div>
</div>
</div>"#;

        let asset: HashMap<BoeConcept, String> = [
      (
        BoeConcept::Header,
        String::from("BIEN 1 - INMUEBLE (VIVIENDA)"),
      ),
      (
        BoeConcept::Description,
        String::from(
          "FINCA URBANA SITUADA EN VALLADOLID, CALLE MARIANO DE LOS COBOS NUM.90, BAJO-1º",
        ),
      ),
      (
        BoeConcept::CatastroReference,
        String::from("4110202UM5141A0003HH"),
      ),
      (
        BoeConcept::Address,
        String::from("CALLE MARIANO DE LOS COBOS 90"),
      ),
      (BoeConcept::PostalCode, String::from("47014")),
      (BoeConcept::City, String::from("VALLADOLID")),
      (
        BoeConcept::Province,
        String::from("Valladolid"),
      ),
      (
        BoeConcept::PrimaryResidence,
        String::from("Sí"),
      ),
      (
        BoeConcept::OwnerStatus,
        String::from("No consta"),
      ),
      (
        BoeConcept::Visitable,
        String::from("No consta"),
      ),
      (
        BoeConcept::RegisterInscription,
        String::from("CONSTA EN EL EDICTO"),
      ),
    ]
    .iter()
    .cloned()
    .collect();

        assert_eq!(asset, parse_asset_auction_page(INPUT).unwrap());
    }

    #[test]
    fn parse_management_auction_test() {
        const INPUT: &str = r#"
      <div id="idBloqueDatos2">
      <h3>Datos de la autoridad gestora</h3>
      <table>
        <tr>
          <th>C&#xF3;digo</th>
          <td>3003000230</td>
        </tr>
        <tr>
          <th>Descripci&#xF3;n</th>
          <td>UNIDAD SUBASTAS JUDICIALES MURCIA<strong> (Ministerio de Justicia)</strong></td>
        </tr>
        <tr>
          <th>Direcci&#xF3;n</th>
          <td>AV DE LA JUSTICIA S/N S/N   ; 30011 MURCIA</td>
        </tr>
        <tr>
          <th>Tel&#xE9;fono</th>
          <td>968833360</td>
        </tr>
        <tr>
          <th>Fax</th>
          <td>-</td>
        </tr>
        <tr>
          <th>Correo electr&#xF3;nico</th>
          <td>subastas.murcia@justicia.es</td>
        </tr>
      </table>
    </div>"#;

        let mgm: HashMap<BoeConcept, String> = [
            (BoeConcept::Code, String::from("3003000230")),
            (
                BoeConcept::Description,
                String::from("UNIDAD SUBASTAS JUDICIALES MURCIA (Ministerio de Justicia)"),
            ),
            (
                BoeConcept::Address,
                String::from("AV DE LA JUSTICIA S/N S/N   ; 30011 MURCIA"),
            ),
            (BoeConcept::Telephone, String::from("968833360")),
            (BoeConcept::Fax, String::from("-")),
            (
                BoeConcept::Email,
                String::from("subastas.murcia@justicia.es"),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(mgm, parse_management_auction_page(INPUT).unwrap());
    }

    #[test]
    fn parse_auction_test() {
        const INPUT: &str = r#"
        <div id="idBloqueDatos1">
        <h3>Datos de la subasta</h3>
        <table>
          <tr>
            <th>Identificador</th>
            <td>
              <strong>SUB-NE-2020-465937</strong>
            </td>
          </tr>
          <tr>
            <th>Tipo de subasta</th>
            <td>
              <strong>NOTARIAL EN VENTA EXTRAJUDICIAL</strong>
            </td>
          </tr>
          <tr>
            <th>Fecha de inicio</th>
            <td>14-07-2020 18:00:00 CET  (ISO: 2020-07-14T18:00:00+02:00)</td>
          </tr>
          <tr>
            <th>Fecha de conclusi&#xF3;n</th>
            <td><strong class="destaca">03-08-2020 18:00:00 CET </strong> (ISO: 2020-08-03T18:00:00+02:00)</td>
          </tr>
          <tr>
            <th>Cantidad reclamada</th>
            <td>81.971,57 &#x20AC;</td>
          </tr>
          <tr>
            <th>Lotes</th>
            <td>Sin lotes</td>
          </tr>
          <tr>
            <th>Anuncio BOE</th>
            <td>BOE-B-2020-21708</td>
          </tr>
          <tr>
            <th>Valor subasta</th>
            <td>75.127,00 &#x20AC;</td>
          </tr>
          <tr>
            <th>Tasaci&#xF3;n</th>
            <td>75.127,00 &#x20AC;</td>
          </tr>
          <tr>
            <th>Puja m&#xED;nima</th>
            <td>Sin puja m&#xED;nima</td>
          </tr>
          <tr>
            <th>Tramos entre pujas</th>
            <td>Sin tramos</td>
          </tr>
          <tr>
            <th>Importe del dep&#xF3;sito</th>
            <td>3.756,35 &#x20AC;</td>
          </tr>
        </table>
      </div>
        "#;

        let auction: HashMap<BoeConcept, String> = [
            (BoeConcept::Identifier, String::from("SUB-NE-2020-465937")),
            (
                BoeConcept::AuctionKind,
                String::from("NOTARIAL EN VENTA EXTRAJUDICIAL"),
            ),
            (
                BoeConcept::StartDate,
                String::from("14-07-2020 18:00:00 CET  (ISO: 2020-07-14T18:00:00+02:00)"),
            ),
            (
                BoeConcept::EndDate,
                String::from("03-08-2020 18:00:00 CET  (ISO: 2020-08-03T18:00:00+02:00)"),
            ),
            (BoeConcept::ClaimQuantity, String::from("81.971,57 €")),
            (BoeConcept::Lots, String::from("Sin lotes")),
            (BoeConcept::Notice, String::from("BOE-B-2020-21708")),
            (BoeConcept::AuctionValue, String::from("75.127,00 €")),
            (BoeConcept::Appraisal, String::from("75.127,00 €")),
            (BoeConcept::MinimumBid, String::from("Sin puja mínima")),
            (BoeConcept::BidStep, String::from("Sin tramos")),
            (BoeConcept::DepositAmount, String::from("3.756,35 €")),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(auction, parse_main_auction_page(INPUT).unwrap());
    }

    #[test]
    fn parse_result_page_test() {
        const INPUT: &str = r#"
<div class="listadoResult">
  <ul>
    <li class="resultado-busqueda">
      <h3>
        SUBASTA SUB-JA-2020-146153</h3>
      <h4>JUZGADO 1ª INST E INSTRUCC. 6 - TORRELAVEGA</h4>
      <p>
        Expediente: 0048/18</p>
      <p>
        Estado: Celebrándose - [Conclusión prevista: 19/07/2020 a las 16:24:28] 
        </p>
      <p>URBANA, TERRENO EN EL PUEBLO DE COBRECES, AYUNTAMIENTO DE ALFOZ DE LLOREDO, BARRIO DE EL PINO, QUE TIENE UN SUPERCIE DE 134 METROS CUADRADOS. CONTINEN DENTRO DE SÍ UN EDIFICIO QUE OCUPA SOBRE EL TRERRENO 122 METROS CUADRADOS APROXIMADAMENTE. ALBERGA UNA UNICA VIVIENDA UNIFAMILIAR. LA PLANTA BAJA SOBRE LA RASANTE DEL TERRENO SE DESTINA A VIVIENDA , CON UNA SUPERFICIE CONSTRUIDA DE 122 METROS CUADRADOS, APROXIMADAMENTE, QUE SE DISTRIBUYE EN COCINA , BAÑO, DESPACHO, SALÓN, SALA Y TERRAZA; LA PLANTA PRIMERA SE DESTINA A VIVIENDA , CON UNA SUPERFICIE COPNSTRUIDA DE 135 METROS CUADRADOS, APROXIMADAMENTE Y SE RPARTE EN DISTRIBUIDOR, BAÑO, CINCO DORMITORIOS Y TERRAZA, Y LA PLANTA BAJO CUBIERTA, SE DESTINA A ESPACIO DIÁFANO , TIENE UNA SUPERFICIE CONSTRUIDA DE 65 METROS CUADRADOS , APROXIMADAMENTE.</p>
      <a href="./detalleSubasta.php?idSub=SUB-JA-2020-146153&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,," class="resultado-busqueda-link-defecto" title="Subasta SUB-JA-2020-146153"> </a>
      <ul>
        <li class="puntoHTML">
          <a href="./detalleSubasta.php?idSub=SUB-JA-2020-146153&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,," class="resultado-busqueda-link-otro" title="Subasta SUB-JA-2020-146153">Más... (Referencia SUB-JA-2020-146153)</a>
        </li>
      </ul>
    </li>
    <li class="resultado-busqueda">
      <h3>
        SUBASTA SUB-JA-2020-149625</h3>
      <h4>JUZGADO 1ª INST E INSTRUCC. 1 - MOTILLA PALANCAR</h4>
      <p>
        Expediente: 0008/17</p>
      <p>
        Estado: Celebrándose - [Conclusión prevista: 20/07/2020 a las 18:00:00] 
        </p>
      <p>FINCA 9557 sita en Villanueva de la Jara, Calle Madrigal nº 3. Inscrita en el Registro de la Propiedad de Motilla del Palancar, tomo 1057, libro 74, folio 95.</p>
      <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149625&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,," class="resultado-busqueda-link-defecto" title="Subasta SUB-JA-2020-149625"> </a>
      <ul>
        <li class="puntoHTML">
          <a href="./detalleSubasta.php?idSub=SUB-JA-2020-149625&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,," class="resultado-busqueda-link-otro" title="Subasta SUB-JA-2020-149625">Más... (Referencia SUB-JA-2020-149625)</a>
        </li>
      </ul>
    </li>
    <li class="resultado-busqueda">
      <h3>
        SUBASTA SUB-AT-2020-20R4186001070</h3>
      <h4>U.R. SUBASTAS ANDALUCIA 41 - SEVILLA</h4>
      <p>
        Estado: Celebrándose - [Conclusión prevista: 20/07/2020 a las 18:00:00] 
        </p>
      <p>SOLAR          . CL TAJO 20. 41110 - BOLLULLOS DE LA MITACION (SEVILLA)</p>
      <a href="./detalleSubasta.php?idSub=SUB-AT-2020-20R4186001070&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,," class="resultado-busqueda-link-defecto" title="Subasta SUB-AT-2020-20R4186001070"> </a>
      <ul>
        <li class="puntoHTML">
          <a href="./detalleSubasta.php?idSub=SUB-AT-2020-20R4186001070&amp;idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,," class="resultado-busqueda-link-otro" title="Subasta SUB-AT-2020-20R4186001070">Más... (Referencia SUB-AT-2020-20R4186001070)</a>
        </li>
      </ul>
    </li>
 </ul>
</div>"#;

        let links = vec![
            BASE_BOE_URL.to_owned() + "./detalleSubasta.php?idSub=SUB-JA-2020-146153&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,",
            BASE_BOE_URL.to_owned() + "./detalleSubasta.php?idSub=SUB-JA-2020-149625&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,",
            BASE_BOE_URL.to_owned() + "./detalleSubasta.php?idSub=SUB-AT-2020-20R4186001070&idBus=_SGFOTnU2NVlnSUwvd2czQzBFcHdoUDFlZTZGS1pLT1lwNm5pbmNIdmNGTXpLNUpZcXNGRElabzlLSGdEckkwL1NuQmpKT3lSd3Z2QTJiM0dPTURUNXBYOEhSNzhqRG5CdExSSXFxZkZSM1phdTh2bkIwUjRXaWFwdkJ2ZzNmVmV0NWc5NjJpU2FDdHQ1amc1SHJSUmhGTGFSTkk4dlFkSWYwTXA5ckFaRUh2TWtkcjM4UmFVY3VCa1JOcklEdWFDdFZpcC81Z0I4UVVYRDdqQjhLeW9RZ2R3aHpOMzRXY1cyZWJwZWRKSXY2RkRHRndmL2JIUXFQckVHdVYzUEh6VA,,"];

        assert_eq!(links, parse_result_page(INPUT));
    }
}
