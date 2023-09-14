
if exist source.zip (
    del source.zip
)
C:/Users/marin/Documents/GitHub/gloom-rs/vendor/7za.exe -tzip a source.zip ^
    Cargo.lock ^
    Cargo.toml ^
    src ^
    shaders ^
    resources/* ^
    -x!resources/helicopter.obj ^
    -x!resources/lunarsurface.obj ^
    -x!resources/.gitkeep
pause
