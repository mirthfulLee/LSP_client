import pathlib


def uri_for():
    """Returns the uri corresponsing to a file in the example workspace."""
    base_dir = pathlib.Path(
        __file__, "..", "..", "examples", "servers", "workspace"
    ).resolve()

    def fn(*args):
        fpath = pathlib.Path(base_dir, *args)
        return uris.from_fs_path(str(fpath))

    return fn
