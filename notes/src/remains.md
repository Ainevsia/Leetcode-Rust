# 39. 组合总和

```cpp
class Solution {
public:
    vector<vector<int>>res;
    vector<int>cur;vector<int>candidates;
    int s=0,t;
    void bt(int start){
        if(s>=t){if(s==t)res.push_back(cur);return;}
        for(int j=start;j<candidates.size();j++){int i=candidates[j];
            cur.push_back(i);s+=i;
            bt(j);
            cur.pop_back();s-=i;
        }
    }
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        t=target;this->candidates=candidates;bt(0);return res;
    }
};
```

# 40.组合总和II

```cpp
class Solution {
public:
    vector<vector<int>>res;
    vector<int>cur;vector<int>candidates;
    int s=0,t;
    void bt(int start){
        if(s>=t){if(s==t)res.push_back(cur);return;}
        for(int j=start;j<candidates.size();j++){
            int i=candidates[j];
            
            if(j>start&&candidates[j]==candidates[j-1])continue;
            cur.push_back(i);s+=i;
            bt(j+1);
            cur.pop_back();s-=i;
        }
    }
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        t=target;sort(candidates.begin(),candidates.end());this->candidates=candidates;bt(0);return res;
    }
};
```

`if(j>start&&candidates[j]==candidates[j-1])continue;`这而想了很久，一直以为是j大于0

为了结果不重复，所以剪枝是必须要进行的操作

# 131.分割回文串

```cpp
class Solution {
public:
    string s;vector<vector<string>>res;vector<string>cur;
    bool valid(int l,int r){
        int lptr=l,rptr=r;while(lptr<=r){if(s[lptr]!=s[rptr])return false;lptr++;rptr--;}return true;
    }
    void bt(int start){
        if(start==s.size()){res.push_back(cur);return;}
        for(int i=start;i<s.size();i++){
            if(valid(start,i)){
                cur.push_back(s.substr(start,i-start+1));
                bt(i+1);
                cur.pop_back();
            }
        }
    }
    vector<vector<string>> partition(string s) {
        this->s=s;bt(0);return res;
    }
};
```

# 93.复原IP地址

https://programmercarl.com/0093.%E5%A4%8D%E5%8E%9FIP%E5%9C%B0%E5%9D%80.html#%E7%AE%97%E6%B3%95%E5%85%AC%E5%BC%80%E8%AF%BE

```cpp
class Solution {
public:
    vector<string> restoreIpAddresses(string s) {

    }
};
```