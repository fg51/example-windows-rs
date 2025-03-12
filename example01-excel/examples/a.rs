use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use anyhow::Result;

use windows::core::PCWSTR;

fn main() -> Result<()> {
    let file_path = "x.xlsx";
    open_and_print_excel(file_path);

    Ok(())
}

fn to_utf16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn open_and_print_excel(file_path: &str) -> Result<()> {
    unsafe {
        // COMライブラリの初期化
        CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED)?;

        // Excelのインスタンスを取得または作成
        let excel_app = get_or_create_excel_instance()?;

        // ファイルパスをVARIANT型に変換
        let file_variant = str_to_bstr_variant(file_path);

        // Workbooksオブジェクトを取得して、ファイルを開く
        let workbooks = excel_app.get("Workbooks")?;
        let workbook = workbooks.invoke("Open", &[file_variant])?;

        // 印刷の実行
        workbook.invoke("PrintOut", &[])?;

        // Excelを終了する（必要なら、excel_app.invoke("Quit", &[])? をコメントアウト）
        excel_app.invoke("Quit", &[])?;

        // COMライブラリのクリーンアップ
        CoUninitialize();
    }
    Ok(())
}

// 既存のExcelインスタンスを取得、または新規作成する関数
fn get_or_create_excel_instance() -> Result<IDispatch> {
    unsafe {
        // 既存のExcelインスタンスを取得
        let prog_id = str_to_bstr("Excel.Application");
        let mut clsid = Default::default();
        CLSIDFromProgID(prog_id.as_ptr(), &mut clsid)?;

        let excel_instance = GetObjectW(ptr::null_mut(), ptr::null_mut(), prog_id.as_ptr());

        // 既存のインスタンスがない場合、新規作成
        if excel_instance.is_err() {
            CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER)
        } else {
            Ok(excel_instance.unwrap())
        }
    }
}

// 文字列をBSTR型のVARIANTに変換するヘルパー関数
fn str_to_bstr_variant(s: &str) -> VARIANT {
    let wide: Vec<u16> = OsString::from(s).encode_wide().collect();
    VARIANT {
        vt: VT_BSTR.0 as u16,
        n1: wide.as_ptr().into(),
        ..Default::default()
    }
}

// 文字列をBSTR型に変換するヘルパー関数
fn str_to_bstr(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = OsString::from(s).encode_wide().collect();
    wide.push(0); // Null-terminated
    wide
}
