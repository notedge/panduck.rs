use fluent_fmt::FluentFormatter;
use fluent_syntax::parser::parse;

#[test]
fn parsing() {
    let mut ctx = FluentFormatter::default();
    let ftl = r#"
### Resource Level Comment
###   * Resource Level Comment2
###   * Resource Level Comment3

# This is a message comment
time-elapsed = Time elapsed: { NUMBER($duration, maximumFractionDigits: 0) }s.

    .placeholder = email@example.com
    .aria-label = Login input value
    .title = Type your login email
"#;
    let out = parse(ftl).unwrap();
    // println!("{:#?}", out);
    println!("{}", ctx.pretty_print(out).unwrap())
}

#[test]
fn parsing2() {
    let mut ctx = FluentFormatter::default();
    let ftl = r#"
# Simple things are simple.
hello-user = Hello, {$userName}!

# Complex things are possible.
shared-photos =
    {$userName}
        {$userName} {$photoCount ->
        [one] added a new photo
       *[other] added {
            $photoCount
        } new photos
    } to {
$userGender ->
        [male] his stream
        [female] her stream
       *[other] their stream
    }.
"#;
    let out = parse(ftl).unwrap();
    // println!("{:#?}", out);
    println!("{}", ctx.pretty_print(out).unwrap())
}

#[test]
fn parsing3() {
    let mut ctx = FluentFormatter::default();
    let ftl = r#"
-brand-name =
{ $case ->
*[nominative] Firefox
[locative] Firefoxa
}

# "Firefox has been successfully updated."
update-successful = { -brand-name } został pomyślnie zaktualizowany.
"#;
    let out = parse(ftl).unwrap();
    // println!("{:#?}", out);
    println!("{}", ctx.pretty_print(out).unwrap())
}
