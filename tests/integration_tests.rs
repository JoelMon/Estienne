use est;
use est::locales::nwt_en::Site::JwOrg;
use pretty_assertions::assert_eq;

#[test]
fn get_scripts_test1(){
    let text: &str = "A popular scripture is John 3:16, it is quoted often.";
    let expected:Vec<String> = vec!["John 3:16".to_string()];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}
#[test]
fn get_scripts_test2(){
    let text: &str = "Two popular scriptures are Genesis 1:1 and John 3:16, they are quoted often.";
    let expected:Vec<String> = vec!["Genesis 1:1".to_string(), "John 3:16".to_string()];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}
#[test]
fn get_scripts_test3(){
    let text: &str = "A popular scripture is re 12:12, it is quoted often.";
    let expected:Vec<String> = vec!["re 12:12".to_string()];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}
#[test]
fn get_scripts_test4(){
    let text: &str = "The following is testing if a fake scripture such as Mary 2:23 is captured.";
    let expected:Vec<String> = vec![];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}
#[test]
fn get_scripts_test5(){
    let text: &str = r#"
    The following is testing if a fake scripture such as Mary 2:23 is captured.
    But really, what I am really interested in is having multiple lines fed into
    `get_scruptures()` to see if it works okay. (Genesis 1:1). This should work,
    but it is always safer to test it. Re 12:12 is used in my tests quite a bit, it seems.

    Let me enter another scripture, Matthew 3:14, here.
    "#;
    let expected:Vec<String> = vec!["Genesis 1:1".into(), "Re 12:12".into(), "Matthew 3:14".into()];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}
#[test]
fn get_scripts_test6(){
    let text: &str = "Testing more than one verse: Psalms 3:1,2,3";
    let expected:Vec<String> = vec!["Psalms 3:1,2,3".into()];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}
#[test]
fn get_scripts_test7(){
    let text: &str = "Testing a range of verses -- Psalms 3:1-3";
    let expected:Vec<String> = vec!["Psalms 3:1-3".into()];
    assert_eq!(expected, est::get_scriptures(text).unwrap());
}


#[test]
fn surround_scripts_test1(){
    let text: &str = "A scripture we should keep in mind is Timothy 3:16.";
    let expected:String = "A scripture we should keep in mind is __Timothy 3:16__.".into();
    assert_eq!(expected, est::surround(text, "__", "__").unwrap());
}
#[test]
fn surround_scripts_test2(){
    let text: &str = "All friends should practice Proverbs 17:17!";
    let expected:String = "All friends should practice <bold>Proverbs 17:17</bold>!".into();
    assert_eq!(expected, est::surround(text, "<bold>", "</bold>").unwrap());
}
#[test]
fn surround_scripts_test3(){
    let text: &str = "Two popular scriptures are Genesis 1:1 and John 3:16, they are quoted often.";
    let expected:String = "Two popular scriptures are **Genesis 1:1** and **John 3:16**, they are quoted often.".into();
    assert_eq!(expected, est::surround(text, "**", "**").unwrap());
}
#[test]
fn surround_scripts_test4(){
    let text: &str = "All friends should practice Proverbs 17:17,18,19!";
    let expected:String = "All friends should practice <bold>Proverbs 17:17,18,19</bold>!".into();
    assert_eq!(expected, est::surround(text, "<bold>", "</bold>").unwrap());
}
#[test]
fn surround_scripts_test5(){
    let text: &str = "All friends should practice Proverbs 17:17-19!";
    let expected:String = "All friends should practice <bold>Proverbs 17:17-19</bold>!".into();
    assert_eq!(expected, est::surround(text, "<bold>", "</bold>").unwrap());
}


#[test]
fn url_scripts_test1(){
    let text: &str = "All friends should practice Proverbs 17:17!";
    let expected:String = "All friends should practice [Proverbs 17:17](https://www.jw.org/en/library/bible/study-bible/books/proverbs/17/#v20017017)!".into();
    assert_eq!(expected, est::url(&JwOrg, text).unwrap());
}