call setup-cef-files.bat

xcopy /Y %CEF_PATH%\Resources\* .
xcopy /Y %CEF_PATH%\Release\* .
