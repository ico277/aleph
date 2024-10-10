# Aleph Null Package Specfication v1
```
file format: .tar.zst
file extension: .null

tree:
/
├── data (REQUIRED) -> Only the following folders are allowed    
│   ├── bin/   (OPTIONAL)
│   ├── etc/   (OPTIONAL)
│   ├── lib/   (OPTIONAL)
│   ├── lib64/ (OPTIONAL)
│   ├── opt/   (OPTIONAL)
│   ├── usr/   (OPTIONAL)
│   └── var/   (OPTIONAL)
├── NULLINFO (REQUIRED) -> Null Package info
├── NULLPOST (OPTIONAL) -> Post-install script
└── NULLPRE  (OPTIONAL) -> Pre-install script   
```

# Null Package Info v1
```
format: TOML
example:
name         = "example"                 (REQUIRED) -> Should be all lowercase with no spaces (use underscores for multiple words), no colons, no >, no <, no =
desc         = "An example package"      (REQUIRED) -> Short and to the point description of this package 
version      = "1.0.0-r1"                (REQUIRED) -> Should be alphabetically and numerically sortable to be able to determine version
provides     = "examplepkg"              (OPTIONAL) -> If not specified, the package name is used
conflicts    = ["other_examplepkg"]      (OPTIONAL) -> Packages that conflict with this package and DO NOT work together, Optionally supports dependency format
dependencies = [">=libexample:1.0.0-r1"] (OPTIONAL) -> List of format: "{one of '<=' '>=', '='}{package name}:{version}"
license      = "Unlicense"               (OPTIONAL) -> SPDX License Identifier
arch         = "x86_64"                  (REQUIRED) -> Can be anything or "any" (defined in package manager config)
```
