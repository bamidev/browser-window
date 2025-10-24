mkdir target\debug
mkdir target\release

xcopy /Y %CEF_PATH%\Resources\* target\debug
xcopy /Y %CEF_PATH%\Resources\* target\release
xcopy /Y %CEF_PATH%\Release\* target\debug
xcopy /Y %CEF_PATH%\Release\* target\release
