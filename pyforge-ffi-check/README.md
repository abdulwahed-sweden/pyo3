# pyforge-ffi-check

This is a simple program which compares ffi definitions from `pyforge-ffi` against those produced by `bindgen`.

If any differ in size, these are printed to stdout and a the process will exit nonzero.

The main purpose of this program is to be run as part of PyForge's continuous integration pipeline to catch possible errors in PyForge's ffi definitions.
