71. Simplify Path
Given an absolute path for a file (Unix-style), simplify it. Or in other words, convert it to the canonical path.
In a UNIX-style file system, a period . refers to the current directory. Furthermore, a double period .. moves the directory up a level. For more information, see: Absolute path vs relative path in Linux/Unix
Note that the returned canonical path must always begin with a slash /, and there must be only a single slash / between two directory names. The last directory name (if it exists) must not end with a trailing /. Also, the canonical path must be the shortest string representing the absolute path.
 
Example 1:
Input: "/home/"
Output: "/home"
Explanation: Note that there is no trailing slash after the last directory name.

Example 2:
Input: "/../"
Output: "/"
Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.

Example 3:
Input: "/home//foo/"
Output: "/home/foo"
Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.

Example 4:
Input: "/a/./b/../../c/"
Output: "/c"

Example 5:
Input: "/a/../../b/../c//.//"
Output: "/c"

Example 6:
Input: "/a//b////c/d//././/.."
Output: "/a/b/c"

C++
class Solution {
public:
    bool processTrailingSlash(string& str) {
        if ( str.empty() || (str.size() == 1 && str.back() == '/') ) {
            return false;
        }
        if ( str.back() == '/' ) {
            str.pop_back();
            return true;
        }
        return false;
    }
    bool processConsecutiveSlashes(string& str) {
        int pos = 0;
        while (pos + 1 <= str.size() - 1) { // next exist
            if (str[pos] == '/' && str[pos+1] == '/') {
                str.erase(pos,1);
                return true;
            } else {
                pos++;
            }
        }
        return false;
    }
    bool processSingleDot(string& str) {
        int pos = str.size() - 1;
        while (pos > 1) {
            if (str[pos] == '.' && str[pos-1] != '.') {
                str.erase(pos, 1);
                return true;
            } else pos--;
        }
        return false;
    }
    string simplifyPath(string path) {
       
    }
};
Give up this method

C++
class Solution {
public:
    string simplifyPath(string path) {
        string buf;
        vector<string> stk;
        stringstream ss(path);
        while (getline(ss, buf, '/')) {
            if (buf == "" || buf == ".") continue;
            if (buf == ".." and not stk.empty()) stk.pop_back();
            else if (buf != "..") stk.push_back(buf);
        }
        string res;
        for (auto str: stk) {
            res += '/' + str;
        }
        return res.empty() ? "/" : res;
    }
};

我现在还没办法直接在文档里写代码
