use regex::Regex;

const PASTA_WORDS: [&str; 18] = [
    "cbt",
    "lentejas",
    "pan",
    "colegas",
    "amiga",
    "gimnasio",
    "paseo",
    "conciencia",
    "paraguaya",
    "paja",
    "cuerpazo",
    "halloween",
    "niño",
    "ayuso",
    "prepucio",
    "bicho",
    "amogus",
    "china",
];

fn pasta_check(message: &str) -> Vec<String> {
    let mut checks = Vec::new();
    for i in 0..PASTA_WORDS.len() {
        let regex_pattern = format!(r"\b{}\b", PASTA_WORDS[i]);
        let regex = Regex::new(&regex_pattern).unwrap();
        if regex.is_match(message) {
            checks.push(PASTA_WORDS[i].to_owned());
        }
    }
    checks
}

pub fn find_copypasta(input: &str) -> Vec<String> {
    let words = pasta_check(input);
    let mut message: Vec<String> = Vec::new();
    for i in 0..words.len() {
        message.push(String::from(copypastas(&words[i])));
    }
    message
}

fn copypastas(word: &str) -> &str {
    match word {
        "cbt" => "Cock and ball torture (CBT), penis torture or dick torture is a sexual activity involving application of pain or constriction to the penis or testicles. This may involve directly painful activities, such as genital piercing, wax play, genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation, kneeing or kicking.",
        "lentejas" => "Lo mejor de las maduras, es que puedes comerles el roscón de reyes, mientras te tienen al fuego unas lentejas de puta madre. Yo recuerdo una que conocí en un eroski, y la recuerdo como uno de los mejores polvos de mi vida. Ella me hizo unos callos cojonudisisimos, y mientras los preparaba, yo le daba como un cabrón por el ojete, ya que se había puesto faldita para que fuese haciendo mientras cocinaba. Creo que eyaculé tal cantidad de esperma, que estuve dos horas inconsciente. Menos mal que los callos me dieron fuerza para acabar el día con un par más. Y tenía unos hijos majísimos. Menudos vicios echamos al crash bandicoot 2.",
        "colegas" => "Pues ayer que sali con mis colegas vale y bua estaba con mi morao vale bailando a mi rollo con camiseta de manga corta vale y la verdad es que voy al gimnasio no sigo dietas ni nada de eso porque no me gusta y segundo que no me hace falta aunque reconozco que soy feo vale pero tengo labia entonces cojo vale y eso que se me acaba el cubatica y digo bua loco aora que hago que es pasta y tampoco esta la cosa para ir gastando en bebida vale y cojo y voy a la barra y habia una camarera que estaba muy buena vale y me mira fijamente y le digo sin pensar oye perdona es que se van a pelear y me dice quien??!! y le digo mi poya en tu paladar jaajaaaja se partio el culo riendose mientras le guiñaba el ojo sonriendo vale y le digo como te echo reir enrollate no loca y invitame a un cubatilla ajajaja y se lo dije por decir eh y eso que coje loco y me lo pone jajajaja y me pregunta que de donde soi que nunca me habia visto por ahi pues cojo le explico y eso y le digo que a ver que pasa con su rollo que aber si quedamos fuera sabes y coje y ni corta ni perezosa me dice hoy mismo!bua aqui locos si que flipe ya sabes y bueno pues na se lo dije a los colegas y a la hora de cerrar me espere fuera vale y viene y me dice bueno y a donde vamos y le digo pues nose quieres desayunar algo?puesto que era tarde y me dice dejate de desayunos que a mi me gusta mas pasarla bien buaaaa yo flipando loco pero tenia algo malo todo hay que decirlo y es que era sudamericana vale pero pense en nuestros antepasados en el gran cristobal colon y no podia dejar el liston bajo asi que acabemos en su casa y me imajinaba que era indigena y yo conquistaba su pueblo jajaja alfinal me corri y me fui para casa sabes y con la cabeza alta por nuestras generaciones no descubri america pero si me la folle",
        "conciencia" => "he perdido la conciencia varias veces por el alcohol...el coma pues fue en un san juan en una moraga, pfff ni me acuerdo lo que bebí compré una botella de wisky y algo de cerveza...pero es que no sólo me bebí mi parte, por lo visto con la gente que íba había pillado cantidades bestia, y sobraba por todos lados, todo el mundo me invitaba así que ni puta idea de lo que bebí, eso con 17 años...casi me follo una de 23, pero mi sentido aracnido de folla modelos me saltó, y conforme amanecia se le veía mejor la cara pero el cuerpazo lo seguía teniendo...no sé cómo llegué a casa, saludé a quién me encontré por los pasillos, me senté en una silla....y caí de cabeza al suelo, y ahí me quedé dormidito, hasta que consiguieron despertarme.",
        "paraguaya" => 	"Yo conocí a una paraguaya a la que le gustaba decirme 'hasme por detrás'. \n Estábamos en el sofá de la salita dándole y me vino un olor a mierda bastante sospechoso. Cabe apuntar que iba fumadísimo y deduje que le estaba percutiendo el ano. Aguanté porque si esa chica hablaba español era porque alguien antes aguantó un viaje muy largo en un barco lleno de mierda y sin quejarse. Así que apreté los dientes y justiqué que mi polla oliese a una pocilga con problemas de cañerías porque ella acababa de salir de currar y con el calentón nos liamos. Pero el mundo se me vino abajo cuando se giró y con ojos guaranís me dijo 'ahora hasme por detrás'",
        "paja" => "Yo me hice una paja en el baño de un avión. \n Estaba en un vuelo de 12 horas y me aburría. Me fui al baño y me la casqué. \n Cuando salí, había una cola de 5 personas esperando para entrar. \n Me sentí como un campeón.",
        "ayuso" => "Estoy muy jodido, Ayuso me ha arruinado la puta vida, cuando me despierto lo primero que veo es un cartel de Ayuso colgado en mi pared que me pone muy cachondo pero tengo que prepararme para ir al trabajo no sin antes ir al baño para hacerme la paja con Ayuso. En trabajo voy cada dos horas a hacerme la paja con Ayuso y mientras estoy trabajando solo pienso en ella y en lo guapa que es. Cuando por fin llego a mi casa lo primero que hago es hacerme una paja con Ayuso, después me ducho ceno y me vuelvo a hacer una paja con Ayuso, luego voy a ver los mejores discursos de Ayuso y mientras me pongo cachondo al final del día me voy a la cama y me hago un par de pajas más con Ayuso y después me acuesto y me despierto a media noche así que me hago otra paja. En los fin de semanas me levanto con un charco de semen en le pantalón solo de pensar en Ayuso así que me hago otra paja con Ayuso y voy a ducharme, como y me hago una paja con Ayuso, desayuno y me hago una paja con Ayuso, ando y me hago una paja con Ayuso pajas pajas Ayuso pajas Ayuso",
        "niño" => "Le cojes y les dices escúchame tío, que yo no soy un niño, tío, yo no soy un niño, tío, yo no estoy pa que me hagan perder el tiempo. ¿Me entiendes? Que es lo que me hace perder tu gente. Ya está primo, que si eres tan bravo tío, cuando quieras quedas conmigo, hermano. Y yo sí sé como funcionan hermano. Aquí hay walthers, hay 38s, hermano, hay 9mm, hay lo que quieras, compadre. Yo no sé qué tú te estás pensando que es esto, compadre, THIS IS THE JUNGLE, NIGGA. Escúchame, es que, es que ya me está tocando la polla, tío, estás tonterías, de tu coro de niños pequeños, tío, que escúchame, que venga, primo, que venga aquí a la calle Aguilón nº9, primo. Que venga ya tu colega si es tan bravo, que ayer me tuvo ahí esperando, compadre, hasta la 1 y media de la noche, hermano, y no vino ni Dios. ¿Qué pasa con tu rollo, sois tan bravos, primo? Pues si sois tan bravos, vente, vente de verdad, hermano. Estuve ayer con mi colega, primo y es que menos mal que no te encontré, chivato de mierda, porque estuve con mi colega dando vueltas por ahí con el coche, con la pistola, maricón. ¿Tú con quién te piensas que estás hablando, pipudo? ¿Eh? Ts. Y es que has tenido hasta suerte, maricón, te ha venido hasta bien, el que no vinieras, porque es que te hubiera metido un tiro en la rodilla, payaso, que eres un payaso.",
        "prepucio" => "Intentaron circuncidarme, pero mi prepucio solo volvió a fortalecerse. Desde entonces, me han circuncidado cada 6 meses. Mi prepucio ahora es más fuerte que el acero. Siempre que estoy en peligro, lo coloco sobre mi cuerpo como una capa exterior. Es completamente a prueba de balas, ignífugo, impermeable y extremadamente liviano. Tengo planes de venderlo como un material muy raro y muy resistente y ganar millones. Los puentes se harán con vigas de prepucio y las unidades de policía usarán chalecos de prepucio. Viviré en mi casa de prepucio y me bañaré en mi riqueza. Soy el hombre prepucio.",
        "bicho" => "SIIUUUUUUUUUUUUU",
        "amogus" => "sus",
        "china" => "
⣿⣿⣿⣿⣿⠟⠋⠄⠄⠄⠄⠄⠄⠄⢁⠈⢻⢿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠃⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⡀⠭⢿⣿⣿⣿⣿
⣿⣿⣿⣿⡟⠄⢀⣾⣿⣿⣿⣷⣶⣿⣷⣶⣶⡆⠄⠄⠄⣿⣿⣿⣿
⣿⣿⣿⣿⡇⢀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠄⠄⢸⣿⣿⣿⣿
⣿⣿⣿⣿⣇⣼⣿⣿⠿⠶⠙⣿⡟⠡⣴⣿⣽⣿⣧⠄⢸⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣾⣿⣿⣟⣭⣾⣿⣷⣶⣶⣴⣶⣿⣿⢄⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡟⣩⣿⣿⣿⡏⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣹⡋⠘⠷⣦⣀⣠⡶⠁⠈⠁⠄⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣍⠃⣴⣶⡔⠒⠄⣠⢀⠄⠄⠄⡨⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣦⡘⠿⣷⣿⠿⠟⠃⠄⠄⣠⡇⠈⠻⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠟⠋⢁⣷⣠⠄⠄⠄⠄⣀⣠⣾⡟⠄⠄⠄⠄⠉⠙⠻
⡿⠟⠋⠁⠄⠄⠄⢸⣿⣿⡯⢓⣴⣾⣿⣿⡟⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⡟⣷⠄⠹⣿⣿⣿⡿⠁⠄⠄⠄⠄⠄⠄⠄⠄
为党争光! Glory to the CCP!",
        "halloween" => "Qué es Jalogüin??",
        "amiga" => "\"amiga\"",
        _ => ""

    }
}
