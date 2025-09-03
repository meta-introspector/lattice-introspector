pub fn print_zos_poem() {
    let zos_axiom_poem = r#"\
In the space of what is, and what could be,
A vector emerges, for all to see.
`zos`, the whisper, the foundational hum,
From which all other numbers come.

Zero, the void, the canvas of creation,
One, the first step, the start of the nation.
Then the primes, in their lonely parade,
Two, three, five, a truth that will not fade.

Seven, the cycle, the rhythm of the spheres,
Eleven, the echo that quiets all fears.
Thirteen, the turning, the shift in the light,
Seventeen, the vision, burning ever so bright.

Nineteen, the threshold, the gateway to the new,
Twenty-three, the pattern, in all that is true.
This is the Zos, the axiom, the key,
To unlock the lattice, for you and for me.
"#;

    let zos_elements_vibe: Vec<(u32, &str)> = vec![
        (0, "The Seed of Naught: A circle drawn, a breath held deep, Where all that is, lies fast asleep."),
        (1, "The Lonely Pillar: A single line, against the grey, The first to stand, to light the way."),
        (2, "The Mirrored Dance: Two points ignite, a vibrant spark, A mirrored dance, within the dark."),
        (3, "The Woven Knot: Three threads combine, a woven knot, The pattern set, the future caught."),
        (5, "The Star of Life: Five fingers spread, a human hand, A five-pointed star, across the land."),
        (7, "The Spiral Path: Seven notes ascend, a winding stair, A spiral path, beyond compare."),
        (11, "The Silver Key: Eleven gates, a whispered word, A truth unseen, a song unheard."),
        (13, "The Serpent's Coil: Thirteen moons, a serpent's coil, A shedding skin, on sacred soil."),
        (17, "The Distant Star: Seventeen lights, a distant fire, A burning hope, a heart's desire."),
        (19, "The Golden Crown: Nineteen years, the sun and moon, A cosmic dance, a silent tune."),
        (23, "The Enigma's Veil: Twenty-three, the number's art, A hidden message, set apart."),
    ];

    println!("{}", zos_axiom_poem);

    println!("\n--- Zos Elements Vibe ---");
    for (number, vibe) in zos_elements_vibe {
        println!("Zos Element {}: {}", number, vibe);
    }
}
