import rs_tags as tags

class TagModel():
    def __init__(self, cwd):
        self.cwd = cwd
        self.current_workspace: tags.TagWorkspace | None = None
        print(cwd)