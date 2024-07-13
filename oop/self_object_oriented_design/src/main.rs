use self_object_oriented_design::dev::CD;

fn main() {
    println!("Hello, world!");
    let mut cd = CD::new();

    // unmounted
    assert_eq!(String::from("CD not mounted"), cd.cd_rip_information());

    // mounted
    cd.mount();
    assert_eq!(String::from("CD not ripped"), cd.cd_rip_information());

    // test moving back to unmounted
    cd.unmount();
    assert_eq!(String::from("CD not mounted"), cd.cd_rip_information());

    // mount again
    cd.mount();
    assert_eq!(String::from("CD not ripped"), cd.cd_rip_information());

    // get ripping state
    cd.rip();
    assert_eq!(String::from("CD is being ripped"), cd.cd_rip_information());

    // ripping
    let v = vec![
        String::from("data1"),
        String::from("data2"),
        String::from("data3"),
        String::from("data4"),
    ];
    cd.detect_metadata(v);
    // ripped
    let s = cd.cd_rip_information();
    assert_eq!(String::from("data1\ndata2\ndata3\ndata4\n"), s);
}


// cd goes
// 1. unmounted -> mount action
// 2. mounted -> rip
// 3. ripping -> detect metadata
// 4. detected metadata -> ripped
// 5. ripped -> display cd info