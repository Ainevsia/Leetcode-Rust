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

```cpp
class Solution {
public:
    vector<string>res;string cur;string s;void bt(int start,int cnt){
        if(cnt==4||start>=s.size()){if(cnt==4&&start==s.size())res.push_back(string(cur.begin(),cur.end()-1));return;}
        for(int i=1;i<=3;i++){
            string sub=string(s.begin()+start,s.begin()+start+i);
            if(valid(sub)){
                auto l=cur.size();
                cur+=sub+".";
                bt(start+i,cnt+1);
                cur.erase(l);
            }
        }
    }
    bool valid(string s){
        if(s.size()==0)return false;
        if(s[0]=='0')return s.size()==1;
        int a=stoi(s);return a>=0 && a<=255;
    }
    vector<string> restoreIpAddresses(string s) {this->s=s;
        bt(0,0);return res;
    }
};
```

# 78.子集

```cpp
class Solution {
public:vector<vector<int>>res;vector<int>cur;vector<int>v;void bt(int start){res.push_back(cur);
    if(start>=v.size())return;for(int i=start;i<v.size();i++){
        cur.push_back(v[i]);bt(i+1);cur.pop_back();
    }
}
    vector<vector<int>> subsets(vector<int>& nums) {
        v=nums;bt(0);return res;
    }
};
```

如果把 子集问题、组合问题、分割问题都抽象为一棵树的话，那么**组合问题和分割问题都是收集树的叶子节点，而子集问题是找树的所有节点**！

# 90.子集II

```cpp
class Solution {
public:vector<vector<int>>res;vector<int>cur;vector<int>v;void bt(int start){res.push_back(cur);
    if(start==v.size())return;for(int i=start;i<v.size();i++){
        if(i>start&&v[i]==v[i-1])continue;
        cur.push_back(v[i]);bt(i+1);cur.pop_back();
    }
}
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        sort(nums.begin(),nums.end());v=nums;bt(0);return res;
    }
};
```

# 491.递增子序列

```cpp
class Solution {
    public:vector<vector<int>>res;
    vector<int>cur;
    vector<int>v;
    void bt(int start){
        if(cur.size()>1)res.push_back(cur);
        if(start>=v.size())return;
        unordered_set<int> uset; 
        for(int i=start;i<v.size();i++){
            if(cur.empty()||v[i]>=cur.back())
            {
                if(uset.find(v[i])!=uset.end())continue;
                uset.insert(v[i]);
                cur.push_back(v[i]);bt(i+1);cur.pop_back();
            }
        }

    }
    vector<vector<int>> findSubsequences(vector<int>& nums) {v=nums;bt(0);return res;}
};

```

本层访问过的元素不再访问，误以为是前后不用重复的就行，需要使用set

# 46.全排列


https://programmercarl.com/0046.%E5%85%A8%E6%8E%92%E5%88%97.html#%E7%AE%97%E6%B3%95%E5%85%AC%E5%BC%80%E8%AF%BE
```cpp
class Solution {
public:
    vector<int>used;
    vector<int>cur;
    vector<int>v;
    vector<vector<int>>res;
    void bt() {
        if(cur.size()==v.size())
        {res.push_back(cur);return;}
        for(int i=0;i<v.size();i++){
            if(used[i]==0){
                used[i]=1;cur.push_back(v[i]);bt();cur.pop_back();used[i]=0;
            }
        }
    }
    vector<vector<int>> permute(vector<int>& nums) {
        v=nums;used=vector<int>(v.size(),0);bt();return res;
    }
};
```
# 47.全排列 II

```cpp
class Solution {
public:
    vector<int>used;
    vector<int>cur;
    vector<int>v;
    vector<vector<int>>res;
    void bt() {
        if(cur.size()==v.size()){
            res.push_back(cur);return;
        }
        for(int i=0;i<v.size();i++){
            if(used[i]==1)continue;
            if(i>0&&v[i]==v[i-1]&&used[i-1]==0)continue;
            used[i]=1;
            cur.push_back(v[i]);
            bt();
            cur.pop_back();
            used[i]=0;
        }
    }
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        v=nums;sort(v.begin(),v.end());
        used=vector<int>(v.size(),0);
        bt();
        return res;
    }
};
```

如何剪枝同一层使用过的：`&&used[i-1]==0`，一下子想不到。

# 332.重新安排行程

```cpp
class Solution {
public:
    unordered_map<string,map<string,int>> targets;
    vector<string> res;
    int ticketNum;
    bool bt() {
        if(res.size()==ticketNum+1)return true;
        for( pair<const string,int>& target: targets[res.back()]) {
            if (target.second > 0) {
                res.push_back(target.first);
                target.second -- ;
                if (bt())return true;
                res.pop_back();
                target.second ++ ;

            }
        }
        return false;
    }
    vector<string> findItinerary(vector<vector<string>>& tickets) {
        for(auto v : tickets) {
            targets[v[0]][v[1]] ++ ;
        }
        res.push_back("JFK");
        ticketNum = tickets.size();
        bt();
        return res;
    }
};
```

这个我是不会的，完全抄了随想录。

本质是，深度优先搜索

做之前心里的一些疑虑：
1. 怎么比较字母序大小，结果直接用map自动帮我排好序
2. 怎么表示这个有向图，然后是用了两个map嵌套

# 51. N皇后

```cpp
class Solution {
public:
    vector<vector<string>> res;
    vector<string> chess;
    int N;
    bool checkpos(int i, int j) {
        for(int k=i+1;k<N;k++){
            if(chess[k][j]=='Q')return false;
        }
        for(int k=0;k<N;k++){
            int x = i+k;
            int y = j+k;
            if(x>=N||y>=N)break;
            if(chess[x][y]=='Q')return false;
        }
        for(int k=0;k<N;k++){
            int x = i+k;
            int y = j-k;
            if(x>=N||y<0)break;
            if(chess[x][y]=='Q')return false;
        }
        return true;
    }
    void bt(int depth) {
        if (depth < 0) {
            res.push_back(chess);
            return;
        }
        for(int pos=0;pos<N;pos++){
            if(checkpos(depth,pos)) {
                chess[depth][pos] = 'Q';
                bt(depth - 1);
                chess[depth][pos] = '.';
            }
        }
    }
    vector<vector<string>> solveNQueens(int n) {
        N=n;
        for (int i=0;i<n;i++){
            chess.push_back(string(n,'.'));
        }
        bt(n-1);
        return res;
    }
};
```

我是从棋盘底部从下往上构造的哈哈

# 37. 解数独

```cpp
class Solution {
public:
    vector<vector<char>> b;
    bool check(int i,int j,int val) {
        for(int k=0;k<9;k++)if(b[i][k]==val||b[k][j]==val)return false;
        int ibase = i/3*3, jbase = j/3*3;
        for(int u=0;u<3;u++)for(int v=0;v<3;v++)if(b[ibase+u][jbase+v]==val)return false;
        return true;
    }
    bool bt(int pos) {
        int i = pos/9, j = pos%9;
        if(i==9)return true;
        if(b[i][j]!='.')return bt(pos+1);
        for(int v=1; v<=9; v++) {
            if(check(i,j,'0'+v)) {
                b[i][j]='0'+v;
                if(bt(pos+1))return true;
                b[i][j]='.';
            }
        }
        return false; // unreachable
    }
    void solveSudoku(vector<vector<char>>& board) {
        b=board;
        bt(0);
        // for(int i=0;i<9;i++){
        //     for(int j=0;j<9;j++){
        //         cout<<b[i][j];
        //     }
        //     cout<<endl;
        // }
        board=b;
    }
};
```

## 贪心算法

# 455. 分发饼干

```cpp
// Custom comparator function for sorting in reverse order
bool reverseComparator(int a, int b) {
    return a > b; // '>' will sort in descending order (reverse), '<' will sort in ascending order
}
class Solution {
public:
    int findContentChildren(vector<int>& g, vector<int>& s) {
        sort(s.begin(),s.end(),reverseComparator);
        sort(g.begin(),g.end(),reverseComparator);
        int cnt = 0;
        int p = 0;
        for (int cookiesize : s) {
            while (p < g.size() && g[p] > cookiesize) p ++ ;
            if (p >= g.size()) break;
            // if (cookiesize >= g[p]) {
                cnt ++ ;
                p ++;
            // }
        }
        return cnt;
    }
};
```

# 376. 摆动序列

```cpp
class Solution {
public:
    int wiggleMaxLength(vector<int>&v) {
        //
        auto tail = unique(v.begin(),v.end());
        v.erase(tail,v.end());
        //
        if(v.size()<=2)return v.size();
        int cnt = 0;
        for (int i = 1; i < v.size() - 1; i ++ ) {
            int pdir = v[i] - v[i-1];
            int cdir = v[i+1] - v[i];
            cnt += pdir*cdir<0?1:0;
        }
        return cnt+2;
    }
};
```

去重之后就不用考虑这么多复杂的情况

# 53. 最大子数组和

```cpp
class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int res = INT_MIN;
        int cnt = 0;
        for (int num : nums) {
            cnt += num;
            res = max(res,cnt);
            if (cnt < 0) cnt = 0;
        }
        return res;
    }
};
```

`res = max(res,cnt);`和`if (cnt < 0) cnt = 0;`这两行顺序一开始搞错了， 导致input只是一个-1的时候有问题

# 122. 买卖股票的最佳时机 II


如果想到其实最终利润是可以分解的，那么本题就很容易了！


```cpp
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int res = 0;
        for ( int i = 1; i < prices.size(); i ++ ) {
            int a = prices[i] - prices[i-1];
            res += a>0?a:0;
        }
        return res;
    }
};
```

# 55. 跳跃游戏

```cpp
class Solution {
public:
    bool canJump(vector<int>& nums) {
        int dst = 0;
        int i = 0;
        while (i <= dst) { 
            dst = max(dst,i+nums[i]);
            i ++ ;
            if (dst >= nums.size() - 1) return true;
        }
        return false;
    }
};
```
# 45.跳跃游戏 II

```cpp
class Solution {
public:
    int jump(vector<int>& nums) {
        int res = 1;
        int predist = nums[0];
        int maxdist = predist;
        int i = 0;
        if(nums.size()==1)return 0;
        if (maxdist>=nums.size()-1) return 1;
        while (i <= maxdist) {
            int nextdist = i+nums[i];
            maxdist=max(maxdist,nextdist);
            if (maxdist>=nums.size()-1) break;
            if (i == predist) {
                res ++ ;
                predist = maxdist;
            }
            i ++ ;

        }
        return res + 1;
    }
};
```

# 1005.K次取反后最大化的数组和

```cpp
class Solution {
public:
    int largestSumAfterKNegations(vector<int>& v, int k) {
        sort(v.begin(),v.end());
        for(int& i: v){
            if (i >= 0) break;
            i=-i;k--;if(k==0)break;
        }if(k==0)return accumulate(v.begin(), v.end(), 0);
        sort(v.begin(),v.end());
        while (k>0){v[0]=-v[0];k--;}
        return accumulate(v.begin(), v.end(), 0);
    }
};
```

# 134. 加油站

```cpp
class Solution {
public:
    int canCompleteCircuit(vector<int>& g, vector<int>& c) {
        int n = g.size();
        vector<int> h(n,0);
        // h[i] -> gas change from i-1 -> i
        h[0] = g[n-1] - c[n-1];
        for (int i = 1 ; i < n ; i ++ ) {
            h[i] = g[i-1]-c[i-1];
        }
        int start = 0;
        int pos = start;
        int csum = 0;
        while (pos < start + n && start < n) {
            csum += h[(1+pos)%n];
            if (csum < 0) {
                start = pos + 1;
                csum = 0;
            }
            pos ++ ;
        }
        return start<n?start:-1;
    }
};
```

贪心算法（方法一） 还挺巧妙的，我这个就是个最大子数组的算法

# 135. 分发糖果

```cpp
class Solution {
public:
    int candy(vector<int>& v) {
        // 1,3,4,5,2
        // 1,2,3,4,1
        // 
        vector<int>res(v.size(),1);
        for (int i = 1 ; i < v.size() ; i ++ ) {
            if (v[i] > v[i-1]) res[i] = res[i-1] + 1;
        }
        for (int i = v.size() - 2; i >= 0 ; i -- ) {
            if (v[i] > v[i+1] && res[i] <= res[i+1]) res[i] = res[i+1] + 1;
        }
        return accumulate(res.begin(),res.end(),0);
    }
};
```

WA了一发漏了`&& res[i] <= res[i+1]`

# 860.柠檬水找零

```cpp
class Solution {
public:
    bool lemonadeChange(vector<int>& bills) {
        int c5 = 0, c10 = 0;
        for ( int bill : bills) {
            switch (bill) {
                case 5:
                    c5 ++ ;
                    break;
                case 10:
                    if (c5 == 0) return false;
                    c5 -- ;
                    c10 ++ ;
                    break;
                default:
                    if (c10>0&&c5>0) {
                        c10 -- ; c5 -- ; continue;
                    }
                    if (c5 >= 3) {
                        c5 -= 3; continue;
                    }
                    return false;
                    break;
            }
        }
        return true;
    }
};
```


模拟题

# 406.根据身高重建队列
错误解答
```cpp

bool f(const vector<int>&a, const vector<int>&b) {
    if (a[1]==b[1])return a[0]>b[0];return a[1]<b[1];
}
class Solution {
public:
    vector<vector<int>> reconstructQueue(vector<vector<int>>& p) {
        vector<vector<int>> res;
        sort(p.begin(),p.end(),f);
        for ( auto peo: p ) {
            if (peo[1]!=0) break;
            res.push_back(peo);
        }
        reverse(res.begin(),res.end());
        for (int i = res.size(); i < p.size(); i ++ ) {
            auto x = p[i];
            int insert_pos = 0; int cnt = 0;
            while ( cnt < x[1] + 1 ) {
                if (p[insert_pos][0] >= x[0]) cnt ++ ;
                insert_pos ++ ;
            }
            res.insert(res.begin()+insert_pos - 1,x);
        }
        return res;
    }
};
```

> 这道题我没有能够做出来

在135. 分发糖果 (opens new window)我就强调过一次，遇到两个维度权衡的时候，一定要先确定一个维度，再确定另一个维度。

如果两个维度一起考虑一定会顾此失彼。

> 我就是错误的按照k来从小到大排序了

```cpp

bool f(vector<int>& a, vector<int>& b){
    if(a[0]==b[0])return a[1]<b[1];return a[0]>b[0];
}
class Solution {
public:
    vector<vector<int>> reconstructQueue(vector<vector<int>>& p) {
        vector<vector<int>> res;
        sort(p.begin(),p.end(),f);
        for (int i = 0; i < p.size(); i ++ ) {
            auto x = p[i];
            int pos = x[1];
            res.insert(res.begin()+pos,x);
        }
        return res;
    }
};
```

先按照身高排序，固定住规律。按照k排序没法获得额外的规律

# 452. 用最少数量的箭引爆气球

想不出来用什么数据结构

```cpp
class Solution {
public:
    int findMinArrowShots(vector<vector<int>>& v) {
        int res = 1;
        sort(v.begin(),v.end());
        for ( int i = 1 ; i < v.size(); i ++ ) {
            if ( v[i][0] > v[i-1][1] ) {
                res ++ ;
            } else {
                v[i][1] = min(v[i][1],v[i-1][1]);
            }
        }
        return res;
    }
};
```

发现不用数据结构，要点是每次右端点取重合的最小值

```plaintext
        ======        
====================
```

# 435. 无重叠区间

```cpp
bool f(const vector<int>&a,const vector<int>&b) {
    if(a[0]==b[0])return a[1]>b[1];return a[0]<b[0];
}
class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& v) {
        int res = 0;
        sort(v.begin(),v.end(),f);
        for ( int i = 1 ; i < v.size(); i ++ ) {
            cout << v[i][0] << v[i][1] << endl;
            if ( v[i][0] >= v[i-1][1] ) {
            } else {
                res ++ ;
                // v[i][1] = min(v[i][1],v[i-1][1]);
            }
        }
        return res;
    }
};
```

不会做，不知道怎么处理这种情况

```plaintext
 -------  -------  ------- -------
====== ======= ====== ======= =====
```

求`需要移除区间的最小数量，使剩余区间互不重叠` -> 求最大的不交叉区间个数

```cpp
bool f(const vector<int>&a,const vector<int>&b) {
    return a[1]<b[1];
}
class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& v) {
        int cnt = 1;
        sort(v.begin(),v.end(),f);
        int rend = v[0][1];
        for ( int i = 1 ; i < v.size(); i ++ ) {
            if (rend <= v[i][0]) {
                rend = v[i][1];
                cnt ++ ;
            }
        }
        return v.size() - cnt;
    }
};
```

通用技巧，求不重合区间的最大个数

# 763.划分字母区间

```cpp
class Solution {
public:
    vector<int> partitionLabels(string s) {
        int v[26] = {0};
        vector<int> res;
        for (int i = 0; i < s.size(); i ++ ) {
            v[s[i]-'a']=i;
        }
        int last = v[s[0]-'a'];
        int pre = 0;
        for (int i = 0; i < s.size(); i ++ ) {
            last = max(last,v[s[i]-'a']);
            if (last == i) {
                res.push_back(i - pre + 1);
                pre = i + 1;
            }
        }
        return res;
    }
};
```

# 56. 合并区间

```cpp
bool f(const vector<int>&a,const vector<int>&b) {
    if(a[0]==b[0])return a[1]<b[1];return a[0]<b[0];
}
class Solution {
public:
    vector<vector<int>>res;
    vector<vector<int>> merge(vector<vector<int>>& v) {
        sort(v.begin(),v.end(),f);
        int l = v[0][0], r = v[0][1];
        for ( int i = 1; i < v.size(); i ++ ) {
            if (v[i][0]<=r) r = max(r,v[i][1]);
            else {
                res.push_back(vector<int>{l, r});
                l = v[i][0];
                r = v[i][1];
            }
        }
        res.push_back(vector<int>{l, r});
        return res;
    }
};
```

这个 很直接

# 738.单调递增的数字

```cpp
class Solution {
public:
    int monotoneIncreasingDigits(int n) {
        vector <int> a;
        int nn = n;
        while(n > 0) {
            int item = n % 10;
            a.push_back(item);
            n /= 10;
        }
        reverse(a.begin(),a.end());
        for (int i = 1; i < a.size(); i ++ ) {
            if (a[i] < a[i-1]) {
                int cnt = a.size() - i;
                int shift = cnt;
                int right = 0;
                while (cnt > 0) {
                    right *= 10;
                    right += 9;
                    cnt -- ;
                }
                int left = 0;
                for (int j = 0; j < i; j ++ ) {
                    left *= 10;
                    left += a[j];
                }
                left = monotoneIncreasingDigits(left - 1);
                return left * 10 * shift + right;
            }
        }
        return nn;
        // 1232 -> 1229
        // 2312 -> 2299
        // 9123 -> 8999
        // 100001 -> 
    }
};
```


332 -- 329 ×
332 -- 299 √
想不出了，原来是要从后往前

```cpp
class Solution {
public:
    int monotoneIncreasingDigits(int n) {
        string s = to_string(n);
        int flag = s.size();
        for ( int i = s.size() - 1; i > 0; i -- ) {
            if (s[i-1] > s[i]) {
                flag = i;
                s[i-1] -- ;
            }
        }
        for ( int i = flag; i < s.size() ; i ++ ) {
            s[i] = '9';
        }
        return stoi(s);
    }
};
```

草 真优雅

# 968.监控二叉树

```cpp
class Solution {
public:
    int cnt = 0;
    bool f(TreeNode*r) { // should not be null
        // if(!r)return true;
        if(!r->left&&!r->right)return false; // no monitor
        bool lres = false;
        if(r->left)res=f(r->left);
        if(!res&&r->right)res=f(r->right);
        if(res == true) return false;
        else {
            cnt ++ ;
            return true;
        }
    }
    int minCameraCover(TreeNode* r) {if(!r->left&&!r->right)return 1;
        f(r);
        return cnt;
    }
};
```

想不明白：在二叉树中如何从低向上推导呢？

```cpp
class Solution {
public:
    int cnt = 0;
    int f(TreeNode*p) {
        // 0 -- 没有覆盖
        // 1 -- 有覆盖了
        // 2 -- 有摄像头
        if(!p)return 1;
        int l = f(p->left);
        int r = f(p->right);
        if (l==1 && r==1) return 0;//////
        if (l==0 || r==0) {//////////////
            cnt ++ ;
            return 2;
        }
        return 1;
    }
    int minCameraCover(TreeNode* r) {if(!r->left&&!r->right)return 1;
        if(f(r)==0)cnt++;///////
        return cnt;
    }
};
```

[0,0,null,null,0,0,null,null,0,0]
  0
 0 n
n 0 
 0 n
n 0
 0