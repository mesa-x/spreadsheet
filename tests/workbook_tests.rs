use mesax::workbook::*;

#[test]
fn test_workbook_creation() {
    let f1 = async {
        let wb = Workbook::new();
        let _ = wb.send_command(WorksheetCommand::Noop).await;

        (
            wb.send_command(WorksheetCommand::Noop).await,
            wb.get_command_count()
        )
    };
    let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    let (res, the_cnt) = runtime.block_on(f1);
    assert!(res.is_ok(), "Got a good response");
    assert!(
        res.unwrap() == CommandResponse::OkResp,
        "Got proper response"
    );
    assert!(the_cnt == 2, "Sent 2 commands")
}
