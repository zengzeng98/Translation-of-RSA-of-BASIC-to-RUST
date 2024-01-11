use std::io::{self, BufRead, BufReader, Write};
use rand::{Rng, thread_rng};
use std::fs::{File, OpenOptions};
use std::{thread, time};


fn verschlueseln(){
    //10
    let mut c:i128=0;
    let mut m:i128 = 0;
    let mut n:i128 =0;
    let mut m_:Vec<i128>=vec![0; 100];
    let zeichenproblock= 3;
    let mut input = String::new();
    //20
    println!("\nFilename der Crypto-Ausgabedatei wo Sie speichern möchten: ");
    io::stdin().read_line(&mut input).expect("Fehlenhaft  bei der Eingabe");
    let aus:String = input.trim().to_string();
    input.clear();
    //25
    let mut file = OpenOptions::new().write(true).append(true).create(true).open(&aus).expect("Funktioniert erfolgreich nicht");

    //30
    println!("Public_Key des Empfängers (Testvoreistellung bei 'RETURN = 94815109'): ");
    io::stdin().read_line(&mut input).expect("Fehlenhaft  bei der Eingabe");
    n = input.trim().parse().unwrap_or(0);
    input.clear();

    //40
    if n==0{ n=94815109; };

    //70
    let mut wir_fahren_nach_berlin = true;
    while wir_fahren_nach_berlin{
        //60
        println!("Zu verschlüsseldem Nachrichetntext eingeben oder NNNN am Zeilenanfang für Ende
Achtung: Keine Eingabe von Kleinbuchstaben!");
        io::stdin().read_line(&mut input).expect("Fehlenhaft  bei der Eingabe");
        let mut m_s= input.trim().to_string();
        input.clear();
        //71
        if m_s.len()>=4{
            let first_four = &m_s[0..4];
            let last_four = &m_s[m_s.len()-4..];
            if first_four == "NNNN" || last_four == "nnnn"{
                println!("Cryptogramm unter Datei: {} abgespeichert.", aus); }
        }
        //80
        let mut q =  m_s.len()/zeichenproblock;
        //90
        //100
        while m_s.len()%3!=0 {
            m_s.push(0 as char);
            q = m_s.len()/zeichenproblock;
        }
        //110
        for i in 0..=q-1{
            //120
            m_[i]=0;
            //130
            for j in 0..zeichenproblock{
                //140
                let a = m_s.chars().nth(3*i+j).unwrap();
                //println!("{}-th von m_s is {}",3*i+j,a);
                //150 160
                m_[i]= m_[i]*100 + (a as u128) as i128;
                //println!("{:?}", m_[i]);
            }//170
        }
        //println!("{:?}", m_);
        //185
        println!("\nCryptogramm:\n");
        //190
        for i in 0..=q-1{
            //200
            m = m_[i];
            //210
            c = m * m;
            c = c - (c/n)*n;
            c = c*m;
            c = c- (c/n)*n;
            //220  //225
            println!("{}:{}",i+1,c);
            println!("");
            writeln!(file, "{}", c).expect("Unable to write to file");
        }
        input.clear();
        println!("Falls Sie noch zusätzlich zu verschlüsselden Nachrichten haben, drüken Sie bitte 1, sonst bitte 0.");
        io::stdin().read_line(&mut input).expect("Fehlenhaft beim Einlesen");
        let weiter:i32 = input.trim().parse().expect("Keine Nummer");
        if weiter==0{
            main();
            wir_fahren_nach_berlin =false;
        }
        else{
            wir_fahren_nach_berlin = true;
        }
        input.clear();
    }

}

fn entschluesseln()-> Result<(), Box<dyn std::error::Error>>{
    let message = "
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
            Programm Entschlüsselung von Nachrichten
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
An diesem Teil benötigen Sie Public- bzw. Secret-keys und den
Filename, wo die verschlüsselte Nachrichten gespeichert werden.
Dann geben Sie noch einen Filename ein, um eine File zu erstellen,
in der die entschlüsselte Nachrichten gespeichert werden.

";
    for c in message.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(30));
    }
    //11
    let zeichenproblock:i128 = 3;
    println!("Eigener Public-Key (Test = 0 +CR = 94815109):");
    //20
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fehlenhaft bei der Eingabe");
    let mut n:i128= input.trim().parse().unwrap_or(0);
    input.clear();
    //30
    if n==0 {n=94815109};
    //40
    println!("Eigener Secret-Key (Test = 0 + CR = 63196467)");
    io::stdin().read_line(&mut input).expect("Fehlenhaft bei der Eingabe");
    let mut d:i128 = input.trim().parse().unwrap_or(0);
    input.clear();
    //50
    if d==0 { d=63196467};
    //51
    println!("Dateiname für die Speicherung des entschlüsselten Texts: ");
    io::stdin().read_line(&mut input).expect("Fehlenhaft bei der Eingabe");
    let aus = input.trim().to_string();
    input.clear();
    //52
    let mut file = OpenOptions::new().write(true).append(true).create(true).open(&aus).expect("Funktioniert erfolgreich nicht");
    //60
    println!("Cryptogramm von Datei: (Keinen manuellen Fall möglich, Grund dafür: die Codes zu chaotisch");
    //70
    //80
    println!("Deswegen geben Sie bitte den Dateiname der Crypto-Datei:");
    io::stdin().read_line(&mut input).expect("Fehlenhaft bei der Eingabe");
    let datei_name = input.trim().to_string();
    input.clear();
    let file1 = File::open(datei_name)?;
    let reader = BufReader::new(file1);
    let mut datei: Vec<i128> = Vec::new();
    for line in reader.lines() {
        let num: i128 = line?.parse()?;
        datei.push(num);
    }
    //81
    let mut text =String::new();
    println!("\nEntschlüsselter Text:");
    for i in datei{
    //100
        let mut c :i128=i;
        let mut d1 = d;
        let mut m :i128= 1;
        //110
        while d1 !=0{
            if d1 -(d1/2)*2==0{
                c = (c*c)%n;
            }
            else{
                m = (m * c) % n;
                c = (c * c) % n;
            }
            d1 /= 2;
        }
        //150
        let mut m_s = m.to_string();
        if !m_s.is_empty() && m_s.starts_with(' ') {
            m_s = m_s[1..].to_string();
        }
        //160
        while m_s.len()%2 !=0{
            m_s = format!("0{}",m_s);
        }
        //170
        let mut zeichen = String::new();
        for t in 0..zeichenproblock {
            let start:usize = t as usize * 2 ;
            let end:usize = start + 2;
            if let Some(substring) = m_s.get(start..end) {
                if let Ok(num) = substring.parse::<u32>() {
                    if let Some(character) = std::char::from_u32(num) {
                        zeichen.push(character);
                    }
                }
            }
        }
        write!(file, "{}", zeichen);
        text.push_str(&zeichen);
    }
    println!("{:?}",text);
    Ok(())
}

fn primzahlen(){
    let message = "
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
      Programm zur Bestimmung des Public- und Secret-Keys
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
Zur Bestimmung des Secret-Keys benötigen Sie 2 (nur Ihnen bekannte)
Primzahlen p und q. Das folgende Programm besteht aus 2 Teilen:
Teil 1 sucht im Umfang einer beliebig gewählten Zahl die naechste
Primzahl, die sich für p oder q eignet.
Teil 2 bestimmt aus den so gefundenen p und q die Secret-und Public-keys

";
    for c in message.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(25));
    }
    //110
    //120
    println!("Zu analysierende Zahl für Übergang Programmteil(0 ist für die Erzeugung der Public-SecretKeys): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("n");
    let mut n :i128 = input.trim().parse().expect("n");
    input.clear();
    //130
    fn prime(){
        let mut input = String::new();
        println!("Start Teil 2: ");
        println!("Welchen Wert hatten Sie für p bestimmt: ");
        io::stdin().read_line(&mut input).expect("p");
        let p:i128 = input.trim().parse().expect("p");
        input.clear();
        println!("Welchen Wert hatten Sie fuer q bestimmt: ");
        io::stdin().read_line(&mut input).expect("q");
        let q:i128 = input.trim().parse().expect("q");
        input.clear();
        println!("Ihr Public-Key lautet :{}", p*q);
        println!("Ihr Secret-Key lautet :{}\n", (2*(p-1)*(q-1)+1)/3);
        println!("Weiterer Schritt: 1: Homepage
                  2: Quit");
        io::stdin().read_line(&mut input).expect("12");
        let quit:i32 = input.trim().parse().expect("12");
        if quit == 1{
            main();
        }
    }
    if n ==0 {
        prime();
    }
    //140
    if n>99999999{
        println!("Bitte einen kleineren Wert waehlen: ");
        io::stdin().read_line(&mut input).expect("n");
        n  = input.trim().parse().expect("n");
        input.clear();
    }
    //150
    if n % 2 == 0 {
        n -= 1;
    }
    let mut y =0;
    let k=10;
    while y != 1 {
        let mut stop = false;
        for _i in 0..k {
            let mut x = thread_rng().gen_range(2..n-2);
            let mut y = 1;
            let mut p = n - 1;
            while p > 0 {
                if p % 2 != 0 {
                    y = (y * x) % n;
                }
                x = (x * x) % n;
                p /= 2;
            }
            if y == 1 {
                println!("{} ist als Primzahl fuer p oder q geeignet.", n);
                stop = true;
                break;
            }
        }
        if stop == true{
            println!("\nMöchten Sie noch eine Zahl für p oder q testen?
1: Ja
2: Für die Erzeugung der Keys
3: Homepage
");
            io::stdin().read_line(&mut input).expect("p,q");
            let wahl:i32 = input.trim().parse().expect("p,q");
            input.clear();
            if wahl == 3{
                main();
                break;
            }
            else if wahl==1{
                println!("Zu analysierende Zahl für Übergang Programmteil: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("n");
                n  = input.trim().parse().expect("n");
                if n % 2 == 0 { n -= 1;}
                input.clear();
            }
            else if wahl==2{
                prime();
                break;
            }
        }
        else{
            println!("Noch keine geeignete Zahl gefunden. Nun ist die zu testende Zahl {}", n - 2);
            n -= 2; }
    }

}

fn main() {
let message = "\nDrüken Sie bitte
1 für Verschlüsselung von Nachrichten,
2 für Entschlüsselung von Nachrichten,
3 für Bestimmung von Primzahlen.
4 für Bratwurst plus Schwarzbier zum Weihnachten und den neuen Jahr.
";
    for c in message.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(35));
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fehlenhaft  bei der Eingabe");
    if input.trim().parse().unwrap_or(0)==1{
        verschlueseln();
    }
    if input.trim().parse().unwrap_or(0)==2{
        let _=entschluesseln();
    }
    if input.trim().parse().unwrap_or(0)==3{
        primzahlen();
    }
    if input.trim().parse().unwrap_or(0)==4{
        let character0 = char::from_u32(0x1F371);
        let character1 = char::from_u32(0x1F37A);
        let character2 = char::from_u32(0x1F37B);
        let character3 = char::from_u32(0x1F32D);
        println!("Hier Bitte:");
        println!("Have {:?} and have {:?} and have {:?} and have {:?}\n", character0,character1,character2,character3);
        main();
    }
}
