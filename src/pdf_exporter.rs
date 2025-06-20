//! This example gives you a first introduction on how to use pdf-writer.

use crate::analyzer::deduce_team_on_bench;
use crate::Round;
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};
use rand::Rng;
use std::collections::HashMap;

pub fn export_rounds_to_pdf(
    rounds: &Vec<Round>,
    games_map: &HashMap<u8, (String, String)>,
    teams_map: &HashMap<u8, String>,
) -> std::io::Result<()> {
    // Start writing.
    let mut pdf = Pdf::new();
    let mut rng = rand::rng();

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(rng.random_range(0..1000));
    let page_tree_id = Ref::new(rng.random_range(0..1000));
    let font_id = Ref::new(rng.random_range(0..1000));
    // contents
    let content_id = Ref::new(rng.random_range(0..1000));
    // pages
    let page0_id = Ref::new(rng.random_range(0..1000));
    let page1_id = Ref::new(rng.random_range(0..1000));
    let page2_id = Ref::new(rng.random_range(0..1000));
    let page3_id = Ref::new(rng.random_range(0..1000));
    let page4_id = Ref::new(rng.random_range(0..1000));
    let page5_id = Ref::new(rng.random_range(0..1000));
    let font_name = Name(b"F1");

    // Write the document catalog with a reference to the page tree.
    pdf.catalog(catalog_id).pages(page_tree_id);

    // Write the page tree with a single child page.
    pdf.pages(page_tree_id)
        .kids([page0_id, page1_id, page2_id, page3_id, page4_id, page5_id])
        .count(6);

    // Write the cover page
    let mut page = pdf.page(page0_id);

    // Set the size to A4 (measured in points) using `media_box` and set the
    // text object we'll write later as the page's contents.
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.parent(page_tree_id);
    page.contents(content_id);

    // We also need to specify which resources the page needs, which in our case
    // is only a font that we name "F1" (the specific name doesn't matter).
    page.resources().fonts().pair(font_name, font_id);
    page.finish();

    // Specify the font we want to use. Because Helvetica is one of the 14 base
    // fonts shipped with every PDF reader, we don't have to embed any font
    // data.
    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));

    // Write a line of text, with the font specified in the resource list
    // before, at a font size of 14.0, starting at coordinates (108.0, 734.0)
    // measured from the bottom left of the page.
    //
    // Because we haven't specified any encoding when writing the Type 1 font,
    // the standard encoding is used which happens to work with most ASCII
    // characters.
    let mut content = Content::new();
    content.set_font(font_name, 28.0);
    content.begin_text();
    content.next_line(108.0, 734.0);
    content.show(Str(b"Tirage du Parcathlon 2025 !"));
    content.end_text();

    content.set_font(font_name, 20.0);
    content.begin_text();
    content.next_line(108.0, 700.0);
    content.show(Str(b"Ci-dessous le programme des 5 rounds"));
    content.end_text();

    pdf.stream(content_id, &content.finish());

    // iterate through pages for rounds
    let pages_ids = [page1_id, page2_id, page3_id, page4_id, page5_id];
    for i in 0..rounds.len() {
        let page_id_for_round = pages_ids[i];
        let mut page = pdf.page(page_id_for_round);
        let round = rounds[i].clone();
        let team_on_bench = deduce_team_on_bench(&round);

        page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
        page.parent(page_tree_id);
        let content_id = Ref::new(rng.random_range(0..1000));
        page.contents(content_id);

        page.resources().fonts().pair(font_name, font_id);
        page.finish();

        let mut y = 750.0;
        let mut content = Content::new();
        content.set_font(font_name, 18.0);
        content.begin_text();
        content.next_line(108.0, y);
        let round_id = i + 1;
        let round_at = format!("Round #{round_id}");
        content.show(Str(round_at.as_bytes()));
        content.end_text();

        y -= 20.0;
        content.begin_text();
        content.next_line(108.0, y);

        match team_on_bench {
            Some(team_on_bench) => {
                let team_on_bench_label = teams_map.get(&team_on_bench).unwrap();
                let on_bench_text = format!("L'equipe en autonomie est : {team_on_bench_label}");
                content.show(Str(on_bench_text.as_bytes()));
            }
            _ => {
                content.show(Str(b"Aucune equipe en autonomie pour ce round"));
            }
        };
        content.end_text();

        round.iter().for_each(|game_pairs| {
            y -= 30.0;
            content.set_font(font_name, 16.0);
            content.begin_text();
            content.next_line(138.0, y);
            for (game, (team1, team2)) in game_pairs {
                let game_labels = games_map.get(&game).unwrap();
                let name = &game_labels.0;
                let person = &game_labels.1;
                let game_text = format!("{name} avec {person}");
                content.show(Str(game_text.as_bytes()));
                content.end_text();

                y -= 20.0;
                content.set_font(font_name, 16.0);
                content.begin_text();
                content.next_line(138.0, y);
                let team1_label = teams_map.get(&team1).unwrap();
                let team2_label = teams_map.get(&team2).unwrap();
                let teams_text = format!("Equipe {team1_label} vs {team2_label}");
                content.show(Str(teams_text.as_bytes()));
                content.end_text();
            }
        });

        pdf.stream(content_id, &content.finish());
    }

    // Finish writing (this automatically creates the cross-reference table and
    // file trailer) and retrieve the resulting byte buffer.
    let buf: Vec<u8> = pdf.finish();

    // Write the thing to a file.
    std::fs::write("target/parcathlon.pdf", buf)
}
