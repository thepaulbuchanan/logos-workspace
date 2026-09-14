import Lake
open Lake

package «semantic_validation_engine» where
  version := "2.2.0"
  keywords := #["logic", "epistemology", "linter"]

lean_lib «SveCore» where
  srcDir := "Src"

@[default_target]
lean_exe «sve_compiler» where
  root := `Src.Main
