import rs_tags as tags
import os

from typing import Dict

class TagModel():
    def __init__(self, cwd):
        self.cwd = cwd
        self.current_workspace: tags.TagWorkspace | None = None

        self.current_workspace = tags.TagWorkspace.open_or_create_workspace(self.cwd, "test")
        self.current_workspace.scan_for_tagfiles()
        print(f"WORKSPACE IS {self.current_workspace}")
        print(cwd)

    def get_tag_mapping_in_dir_as_strings(self, path_to_directory: str) -> Dict[str, str]:
        print(f"START get_tag_mapping_in_dir_as_strings: {path_to_directory}")
        if self.current_workspace == None:
            return {}
        return_val = {}
        for entry in os.listdir(path_to_directory):
            full_path = os.path.join(path_to_directory, entry)
            list_tag = self.current_workspace.get_tags_for_file_name(full_path)
            if len(list_tag) <= 0:
                continue
            tags_as_list = []
            for tag in list_tag:
                if tag.is_simple():
                    tags_as_list.append(tag.simple_value)
                else:
                    tags_as_list.append(f"{tag.kv_key}: {tag.kv_value}")
            print(tags_as_list)
            return_val[entry] = ", ".join(tags_as_list)
        
        print(f"DONE get_tag_mapping_in_dir_as_strings: {path_to_directory}")
        return return_val
        
    def get_tags_for_filename_as_list_of_str(self, path_to_file_name: str) -> list[str]:
        print(f"START get_tags_for_filename_as_list_of_str: {path_to_file_name}")
        # return ["Hello", "123: 124"]
        if self.current_workspace:
            list_tag = self.current_workspace.get_tags_for_file_name(path_to_file_name)
            if len(list_tag) <= 0:
                return []
            return_list = []
            for tag in list_tag:
                if tag.is_simple():
                    return_list.append(tag.simple_value)
                else:
                    return_list.append(f"{tag.kv_key}: {tag.kv_value}")
            print(f"DONE get_tags_for_filename_as_list_of_str: {path_to_file_name}")
            return return_list
        else:
            return []