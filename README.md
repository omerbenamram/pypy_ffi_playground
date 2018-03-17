To generate bindings, some pypy macros are required (included in `/include`).
I've used `bindgen wrapper.h -o ./src/cpyext_ffi -- -I./Include`

The main pain point is we need to manually rename the functions from their pypy expanded form
`PyPy..` to `Py` to keep convention (note that the `link` command are still valid for `libpypy3-c`)

I've used this simple script (yes - it should probably be included in the `build.rs`)

```python
with open('/Users/omerba/Workspace/pypy_ffi/src/ffi_bindgen.rs', 'r') as f:
    src = f.read()

r = re.sub(r"(?!\".*)PyPy(?!.*\")", "Py", src)

with open('/Users/omerba/Workspace/pypy_ffi/src/ffi_bindgen.rs', 'w') as f:
    f.write(r)
```

(also had to renamed like 3 functions manually back to PyPy since they were included twice)