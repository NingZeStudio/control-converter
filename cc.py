#!/usr/bin/env python3
"""Convert control-layout JSON between Zalith Launcher 2 and FoldCraftLauncher.

Usage:
  python control_converter.py zl2fcl input.json output.json
  python control_converter.py fcl2zl input.json output.json --include-directions
  python control_converter.py fcl2zl input.json output.json --lossless --absolute-as-percentage --strip-meta
  python control_converter.py auto input.json output.json
  python control_converter.py api --host 127.0.0.1 --port 8000
  
  
/**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*/
  
/**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*//**
 *                             _ooOoo_
 *                            o8888888o
 *                            88" . "88
 *                            (| -_- |)
 *                            O\  =  /O
 *                         ____/`---'\____
 *                       .'  \\|     |//  `.
 *                      /  \\|||  :  |||//  \
 *                     /  _||||| -:- |||||-  \
 *                     |   | \\\  -  /// |   |
 *                     | \_|  ''\---/''  |   |
 *                     \  .-\__  `-`  ___/-. /
 *                   ___`. .'  /--.--\  `. . __
 *                ."" '<  `.___\_<|>_/___.'  >'"".
 *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *               \  \ `-.   \_ __\ /__ _/   .-` /  /
 *          ======`-.____`-.___\_____/___.-`____.-'======
 *                             `=---='
 *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *                     佛祖保佑        永无BUG
 *            佛曰:
 *                   写字楼里写字间，写字间里程序员；
 *                   程序人员写程序，又拿程序换酒钱。
 *                   酒醒只在网上坐，酒醉还来网下眠；
 *                   酒醉酒醒日复日，网上网下年复年。
 *                   但愿老死电脑间，不愿鞠躬老板前；
 *                   奔驰宝马贵者趣，公交自行程序员。
 *                   别人笑我忒疯癫，我笑自己命太贱；
 *                   不见满街漂亮妹，哪个归得程序员？
*/
"""




from __future__ import annotations

import argparse
import json
import math
import re
import sys
import uuid
from collections import Counter, defaultdict
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.parse import parse_qs, urlparse

FCL_CONTROLLER_VERSION = 21
ZL_EDITOR_VERSION = 11
META_KEY = "_control_byIQge报错别找我"
META_SCHEMA_VERSION = 1

ZL_KEY_ALIASES = {
    "GLFW_MOUSE_BUTTON_1": "GLFW_MOUSE_BUTTON_LEFT",
    "GLFW_MOUSE_BUTTON_2": "GLFW_MOUSE_BUTTON_RIGHT",
    "GLFW_MOUSE_BUTTON_3": "GLFW_MOUSE_BUTTON_MIDDLE",
    "MOUSE_SCROLL_UP": "launcher.event.scroll_up.single",
    "MOUSE_SCROLL_DOWN": "launcher.event.scroll_down.single",
    "key.mouse.left": "GLFW_MOUSE_BUTTON_LEFT",
    "key.mouse.right": "GLFW_MOUSE_BUTTON_RIGHT",
    "key.mouse.middle": "GLFW_MOUSE_BUTTON_MIDDLE",
    "key.mouse.4": "GLFW_MOUSE_BUTTON_4",
    "key.mouse.5": "GLFW_MOUSE_BUTTON_5",
    "key.mouse.6": "GLFW_MOUSE_BUTTON_6",
    "key.mouse.7": "GLFW_MOUSE_BUTTON_7",
    "key.mouse.8": "GLFW_MOUSE_BUTTON_8",
    "key.keyboard.unknown": "GLFW_KEY_UNKNOWN",
    "key.keyboard.num.lock": "GLFW_KEY_NUM_LOCK",
    "key.keyboard.keypad.0": "GLFW_KEY_KP_0",
    "key.keyboard.keypad.1": "GLFW_KEY_KP_1",
    "key.keyboard.keypad.2": "GLFW_KEY_KP_2",
    "key.keyboard.keypad.3": "GLFW_KEY_KP_3",
    "key.keyboard.keypad.4": "GLFW_KEY_KP_4",
    "key.keyboard.keypad.5": "GLFW_KEY_KP_5",
    "key.keyboard.keypad.6": "GLFW_KEY_KP_6",
    "key.keyboard.keypad.7": "GLFW_KEY_KP_7",
    "key.keyboard.keypad.8": "GLFW_KEY_KP_8",
    "key.keyboard.keypad.9": "GLFW_KEY_KP_9",
    "key.keyboard.keypad.add": "GLFW_KEY_KP_ADD",
    "key.keyboard.keypad.decimal": "GLFW_KEY_KP_DECIMAL",
    "key.keyboard.keypad.enter": "GLFW_KEY_KP_ENTER",
    "key.keyboard.keypad.equal": "GLFW_KEY_KP_EQUAL",
    "key.keyboard.keypad.multiply": "GLFW_KEY_KP_MULTIPLY",
    "key.keyboard.keypad.divide": "GLFW_KEY_KP_DIVIDE",
    "key.keyboard.keypad.subtract": "GLFW_KEY_KP_SUBTRACT",
    "key.keyboard.down": "GLFW_KEY_DOWN",
    "key.keyboard.left": "GLFW_KEY_LEFT",
    "key.keyboard.right": "GLFW_KEY_RIGHT",
    "key.keyboard.up": "GLFW_KEY_UP",
    "key.keyboard.apostrophe": "GLFW_KEY_APOSTROPHE",
    "key.keyboard.backslash": "GLFW_KEY_BACKSLASH",
    "key.keyboard.comma": "GLFW_KEY_COMMA",
    "key.keyboard.equal": "GLFW_KEY_EQUAL",
    "key.keyboard.grave.accent": "GLFW_KEY_GRAVE_ACCENT",
    "key.keyboard.left.bracket": "GLFW_KEY_LEFT_BRACKET",
    "key.keyboard.minus": "GLFW_KEY_MINUS",
    "key.keyboard.period": "GLFW_KEY_PERIOD",
    "key.keyboard.right.bracket": "GLFW_KEY_RIGHT_BRACKET",
    "key.keyboard.semicolon": "GLFW_KEY_SEMICOLON",
    "key.keyboard.slash": "GLFW_KEY_SLASH",
    "key.keyboard.space": "GLFW_KEY_SPACE",
    "key.keyboard.tab": "GLFW_KEY_TAB",
    "key.keyboard.left.alt": "GLFW_KEY_LEFT_ALT",
    "key.keyboard.left.control": "GLFW_KEY_LEFT_CONTROL",
    "key.keyboard.left.shift": "GLFW_KEY_LEFT_SHIFT",
    "key.keyboard.left.win": "GLFW_KEY_LEFT_SUPER",
    "key.keyboard.left.super": "GLFW_KEY_LEFT_SUPER",
    "key.keyboard.left.meta": "GLFW_KEY_LEFT_SUPER",
    "key.keyboard.right.alt": "GLFW_KEY_RIGHT_ALT",
    "key.keyboard.right.control": "GLFW_KEY_RIGHT_CONTROL",
    "key.keyboard.right.shift": "GLFW_KEY_RIGHT_SHIFT",
    "key.keyboard.right.win": "GLFW_KEY_RIGHT_SUPER",
    "key.keyboard.right.super": "GLFW_KEY_RIGHT_SUPER",
    "key.keyboard.right.meta": "GLFW_KEY_RIGHT_SUPER",
    "key.keyboard.enter": "GLFW_KEY_ENTER",
    "key.keyboard.escape": "GLFW_KEY_ESCAPE",
    "key.keyboard.backspace": "GLFW_KEY_BACKSPACE",
    "key.keyboard.delete": "GLFW_KEY_DELETE",
    "key.keyboard.end": "GLFW_KEY_END",
    "key.keyboard.home": "GLFW_KEY_HOME",
    "key.keyboard.insert": "GLFW_KEY_INSERT",
    "key.keyboard.page.down": "GLFW_KEY_PAGE_DOWN",
    "key.keyboard.page.up": "GLFW_KEY_PAGE_UP",
    "key.keyboard.caps.lock": "GLFW_KEY_CAPS_LOCK",
    "key.keyboard.pause": "GLFW_KEY_PAUSE",
    "key.keyboard.scroll.lock": "GLFW_KEY_SCROLL_LOCK",
    "key.keyboard.menu": "GLFW_KEY_MENU",
    "key.keyboard.print.screen": "GLFW_KEY_PRINT_SCREEN",
    "key.keyboard.world.1": "GLFW_KEY_WORLD_1",
    "key.keyboard.world.2": "GLFW_KEY_WORLD_2",
    "key.keyboard.keypad.separator": "GLFW_KEY_KP_DECIMAL",
}
ZL_KEY_ALIASES.update({f"key.keyboard.{i}": f"GLFW_KEY_{i}" for i in range(10)})
ZL_KEY_ALIASES.update({f"key.keyboard.{chr(code)}": f"GLFW_KEY_{chr(code).upper()}" for code in range(ord("a"), ord("z") + 1)})
ZL_KEY_ALIASES.update({f"key.keyboard.f{i}": f"GLFW_KEY_F{i}" for i in range(1, 26)})

FCL_MOUSE = {
    "GLFW_MOUSE_BUTTON_LEFT": 1000,
    "GLFW_MOUSE_BUTTON_MIDDLE": 1001,
    "GLFW_MOUSE_BUTTON_RIGHT": 1002,
}
FCL_MOUSE_REVERSE = {
    1000: "GLFW_MOUSE_BUTTON_LEFT",
    1001: "GLFW_MOUSE_BUTTON_MIDDLE",
    1002: "GLFW_MOUSE_BUTTON_RIGHT",
}
FCL_SCROLL_REVERSE = {
    1003: ("launcher.event.scroll_up.single", "launcher.event.scroll_up"),
    1004: ("launcher.event.scroll_down.single", "launcher.event.scroll_down"),
}
ZL_ONLY_KEYS = {
    "GLFW_KEY_WORLD_1",
    "GLFW_KEY_WORLD_2",
    "GLFW_KEY_F25",
    "GLFW_KEY_MENU",
    "GLFW_KEY_LAST",
    "GLFW_MOD_SHIFT",
    "GLFW_MOD_CONTROL",
    "GLFW_MOD_ALT",
    "GLFW_MOD_SUPER",
    "GLFW_MOD_CAPS_LOCK",
    "GLFW_MOD_NUM_LOCK",
    "GLFW_MOUSE_BUTTON_4",
    "GLFW_MOUSE_BUTTON_5",
    "GLFW_MOUSE_BUTTON_6",
    "GLFW_MOUSE_BUTTON_7",
    "GLFW_MOUSE_BUTTON_8",
    "GLFW_MOUSE_BUTTON_LAST",
}
UNSUPPORTED_ZL_KEY_REASONS = {key: "FCL controls do not define an exact matching keycode" for key in ZL_ONLY_KEYS}
UNSUPPORTED_ZL_KEY_EVENTS = set(UNSUPPORTED_ZL_KEY_REASONS)
UNSUPPORTED_FCL_KEY_REASONS = {
    0: "FCL KEY_RESERVED is not a real input key",
    121: "FCL KEY_KPCOMMA has no exact GLFW/ZL control event equivalent",
}

GLFW_TO_FCL = {
    "GLFW_KEY_UNKNOWN": 240,
    "GLFW_KEY_SPACE": 57,
    "GLFW_KEY_APOSTROPHE": 40,
    "GLFW_KEY_COMMA": 51,
    "GLFW_KEY_MINUS": 12,
    "GLFW_KEY_PERIOD": 52,
    "GLFW_KEY_SLASH": 53,
    "GLFW_KEY_0": 11,
    "GLFW_KEY_1": 2,
    "GLFW_KEY_2": 3,
    "GLFW_KEY_3": 4,
    "GLFW_KEY_4": 5,
    "GLFW_KEY_5": 6,
    "GLFW_KEY_6": 7,
    "GLFW_KEY_7": 8,
    "GLFW_KEY_8": 9,
    "GLFW_KEY_9": 10,
    "GLFW_KEY_SEMICOLON": 39,
    "GLFW_KEY_EQUAL": 13,
    "GLFW_KEY_A": 30,
    "GLFW_KEY_B": 48,
    "GLFW_KEY_C": 46,
    "GLFW_KEY_D": 32,
    "GLFW_KEY_E": 18,
    "GLFW_KEY_F": 33,
    "GLFW_KEY_G": 34,
    "GLFW_KEY_H": 35,
    "GLFW_KEY_I": 23,
    "GLFW_KEY_J": 36,
    "GLFW_KEY_K": 37,
    "GLFW_KEY_L": 38,
    "GLFW_KEY_M": 50,
    "GLFW_KEY_N": 49,
    "GLFW_KEY_O": 24,
    "GLFW_KEY_P": 25,
    "GLFW_KEY_Q": 16,
    "GLFW_KEY_R": 19,
    "GLFW_KEY_S": 31,
    "GLFW_KEY_T": 20,
    "GLFW_KEY_U": 22,
    "GLFW_KEY_V": 47,
    "GLFW_KEY_W": 17,
    "GLFW_KEY_X": 45,
    "GLFW_KEY_Y": 21,
    "GLFW_KEY_Z": 44,
    "GLFW_KEY_LEFT_BRACKET": 26,
    "GLFW_KEY_RIGHT_BRACKET": 27,
    "GLFW_KEY_BACKSLASH": 43,
    "GLFW_KEY_GRAVE_ACCENT": 41,
    "GLFW_KEY_ESCAPE": 1,
    "GLFW_KEY_ENTER": 28,
    "GLFW_KEY_TAB": 15,
    "GLFW_KEY_BACKSPACE": 14,
    "GLFW_KEY_INSERT": 110,
    "GLFW_KEY_DELETE": 111,
    "GLFW_KEY_RIGHT": 106,
    "GLFW_KEY_LEFT": 105,
    "GLFW_KEY_DOWN": 108,
    "GLFW_KEY_UP": 103,
    "GLFW_KEY_PAGE_UP": 104,
    "GLFW_KEY_PAGE_DOWN": 109,
    "GLFW_KEY_HOME": 102,
    "GLFW_KEY_END": 107,
    "GLFW_KEY_CAPS_LOCK": 58,
    "GLFW_KEY_SCROLL_LOCK": 70,
    "GLFW_KEY_NUM_LOCK": 69,
    "GLFW_KEY_PRINT_SCREEN": 99,
    "GLFW_KEY_PAUSE": 119,
    "GLFW_KEY_F1": 59,
    "GLFW_KEY_F2": 60,
    "GLFW_KEY_F3": 61,
    "GLFW_KEY_F4": 62,
    "GLFW_KEY_F5": 63,
    "GLFW_KEY_F6": 64,
    "GLFW_KEY_F7": 65,
    "GLFW_KEY_F8": 66,
    "GLFW_KEY_F9": 67,
    "GLFW_KEY_F10": 68,
    "GLFW_KEY_F11": 87,
    "GLFW_KEY_F12": 88,
    "GLFW_KEY_F13": 183,
    "GLFW_KEY_F14": 184,
    "GLFW_KEY_F15": 185,
    "GLFW_KEY_F16": 186,
    "GLFW_KEY_F17": 187,
    "GLFW_KEY_F18": 188,
    "GLFW_KEY_F19": 189,
    "GLFW_KEY_F20": 190,
    "GLFW_KEY_F21": 191,
    "GLFW_KEY_F22": 192,
    "GLFW_KEY_F23": 193,
    "GLFW_KEY_F24": 194,
    "GLFW_KEY_KP_0": 82,
    "GLFW_KEY_KP_1": 79,
    "GLFW_KEY_KP_2": 80,
    "GLFW_KEY_KP_3": 81,
    "GLFW_KEY_KP_4": 75,
    "GLFW_KEY_KP_5": 76,
    "GLFW_KEY_KP_6": 77,
    "GLFW_KEY_KP_7": 71,
    "GLFW_KEY_KP_8": 72,
    "GLFW_KEY_KP_9": 73,
    "GLFW_KEY_KP_DECIMAL": 83,
    "GLFW_KEY_KP_DIVIDE": 98,
    "GLFW_KEY_KP_MULTIPLY": 55,
    "GLFW_KEY_KP_SUBTRACT": 74,
    "GLFW_KEY_KP_ADD": 78,
    "GLFW_KEY_KP_ENTER": 96,
    "GLFW_KEY_KP_EQUAL": 117,
    "GLFW_KEY_LEFT_SHIFT": 42,
    "GLFW_KEY_LEFT_CONTROL": 29,
    "GLFW_KEY_LEFT_ALT": 56,
    "GLFW_KEY_LEFT_SUPER": 125,
    "GLFW_KEY_RIGHT_SHIFT": 54,
    "GLFW_KEY_RIGHT_CONTROL": 97,
    "GLFW_KEY_RIGHT_ALT": 100,
    "GLFW_KEY_RIGHT_SUPER": 126,
}
FCL_TO_GLFW = {v: k for k, v in GLFW_TO_FCL.items()}

ZL_TO_FCL_FALLBACKS = {
    "GLFW_MOUSE_BUTTON_4": (1003, "FCL has no side mouse button 4; substituted with scroll up"),
    "GLFW_MOUSE_BUTTON_5": (1004, "FCL has no side mouse button 5; substituted with scroll down"),
    "GLFW_MOUSE_BUTTON_6": (1003, "FCL has no side mouse button 6; substituted with scroll up"),
    "GLFW_MOUSE_BUTTON_7": (1004, "FCL has no side mouse button 7; substituted with scroll down"),
    "GLFW_MOUSE_BUTTON_8": (1004, "FCL has no side mouse button 8; substituted with scroll down"),
    "GLFW_KEY_F25": (GLFW_TO_FCL["GLFW_KEY_F24"], "FCL has no F25; substituted with F24"),
    "GLFW_KEY_WORLD_1": (GLFW_TO_FCL["GLFW_KEY_UNKNOWN"], "FCL has no WORLD_1; substituted with UNKNOWN"),
    "GLFW_KEY_WORLD_2": (GLFW_TO_FCL["GLFW_KEY_UNKNOWN"], "FCL has no WORLD_2; substituted with UNKNOWN"),
    "GLFW_KEY_MENU": (GLFW_TO_FCL["GLFW_KEY_UNKNOWN"], "FCL has no menu key; substituted with UNKNOWN"),
    "GLFW_KEY_LAST": (GLFW_TO_FCL["GLFW_KEY_UNKNOWN"], "FCL has no LAST sentinel key; substituted with UNKNOWN"),
    "GLFW_MOD_SHIFT": (GLFW_TO_FCL["GLFW_KEY_LEFT_SHIFT"], "FCL has no modifier event; substituted with left shift"),
    "GLFW_MOD_CONTROL": (GLFW_TO_FCL["GLFW_KEY_LEFT_CONTROL"], "FCL has no modifier event; substituted with left control"),
    "GLFW_MOD_ALT": (GLFW_TO_FCL["GLFW_KEY_LEFT_ALT"], "FCL has no modifier event; substituted with left alt"),
    "GLFW_MOD_SUPER": (GLFW_TO_FCL["GLFW_KEY_LEFT_SUPER"], "FCL has no modifier event; substituted with left super"),
    "GLFW_MOD_CAPS_LOCK": (GLFW_TO_FCL["GLFW_KEY_CAPS_LOCK"], "FCL has no modifier event; substituted with caps lock"),
    "GLFW_MOD_NUM_LOCK": (GLFW_TO_FCL["GLFW_KEY_NUM_LOCK"], "FCL has no modifier event; substituted with num lock"),
    "GLFW_MOUSE_BUTTON_LAST": (1004, "FCL has no LAST mouse sentinel; substituted with scroll down"),
}

FCL_TO_ZL_FALLBACKS = {
    0: (("key", "GLFW_KEY_UNKNOWN"), "FCL KEY_RESERVED is not a real input key; substituted with GLFW_KEY_UNKNOWN"),
    121: (("key", "GLFW_KEY_KP_DECIMAL"), "FCL KEY_KPCOMMA has no exact GLFW key; substituted with keypad decimal"),
}

_WARNED_MESSAGES: set[str] = set()
_SUBSTITUTION_COUNTS = {
    "keys": 0,
    "events": 0,
    "layers": 0,
    "directions": 0,
}


def warn(message: str, strict: bool = False, once: bool = False) -> None:
    if strict:
        raise ValueError(message)
    if once:
        if message in _WARNED_MESSAGES:
            return
        _WARNED_MESSAGES.add(message)
    print(f"warning: {message}", file=sys.stderr)


def deep_copy_json(value: Any) -> Any:
    return json.loads(json.dumps(value, ensure_ascii=False))


def get_meta(obj: Any) -> dict[str, Any] | None:
    if isinstance(obj, dict):
        meta = obj.get(META_KEY)
        if isinstance(meta, dict):
            return meta
    return None


def set_meta(obj: dict[str, Any], meta: dict[str, Any] | None) -> dict[str, Any]:
    if meta:
        obj[META_KEY] = meta
    return obj


def substitution(source: dict[str, Any], target: dict[str, Any], reason: str, category: str = "events") -> dict[str, Any]:
    if category in _SUBSTITUTION_COUNTS:
        _SUBSTITUTION_COUNTS[category] += 1
    return {
        "source": deep_copy_json(source),
        "target": deep_copy_json(target),
        "reason": reason,
    }


def append_substitutions(mapping: dict[str, Any] | None, substitutions: list[dict[str, Any]]) -> dict[str, Any] | None:
    if not substitutions:
        return mapping
    result = deep_copy_json(mapping or {})
    existing = result.get("substitutions")
    if not isinstance(existing, list):
        existing = []
    existing.extend(deep_copy_json(substitutions))
    result["substitutions"] = existing
    return result


def make_meta(
    origin_format: str,
    origin_kind: str,
    origin_id: str,
    original: Any,
    mapping: dict[str, Any] | None = None,
) -> dict[str, Any]:
    original_copy = strip_converter_meta(original)
    meta = {
        "schema": META_SCHEMA_VERSION,
        "originFormat": origin_format,
        "originKind": origin_kind,
        "originId": str(origin_id or ""),
        "original": original_copy,
    }
    if mapping:
        meta["mapping"] = deep_copy_json(mapping)
    return meta


def meta_original(obj: Any, expected_format: str, expected_kind: str | None = None) -> Any | None:
    meta = get_meta(obj)
    if not meta:
        return None
    if meta.get("originFormat") != expected_format:
        return None
    if expected_kind is not None and meta.get("originKind") != expected_kind:
        return None
    original = meta.get("original")
    if not isinstance(original, dict):
        return None
    return deep_copy_json(original)


def meta_kind(obj: Any) -> str | None:
    meta = get_meta(obj)
    if not meta:
        return None
    kind = meta.get("originKind")
    return str(kind) if isinstance(kind, str) else None


def short_id() -> str:
    return uuid.uuid4().hex[:12]


def fcl_id() -> str:
    return str(uuid.uuid4())


def text_default(value: Any) -> str:
    if isinstance(value, dict):
        return str(value.get("default", ""))
    if value is None:
        return ""
    return str(value)


def translatable(text: str, source: Any = None) -> dict[str, Any]:
    if isinstance(source, dict):
        default = source.get("default", text or "")
        match_queue = source.get("matchQueue")
        if isinstance(match_queue, list):
            return {
                "default": str(default or ""),
                "matchQueue": deep_copy_json(match_queue),
            }
    return {"default": text or "", "matchQueue": []}


def clamp_int(value: Any, default: int = 0) -> int:
    try:
        return int(round(float(value)))
    except (TypeError, ValueError):
        return default


def clamp_float(value: Any, default: float = 0.0) -> float:
    try:
        result = float(value)
    except (TypeError, ValueError):
        return default
    return result if math.isfinite(result) else default


def clamp_range(value: Any, minimum: float, maximum: float, default: float) -> float:
    return max(minimum, min(maximum, clamp_float(value, default)))


def clamp_zl_dp(value: Any, default: float = 50.0) -> float:
    return max(5.0, clamp_float(value, default))


def clamp_zl_shape(value: Any, default: float = 0.0) -> float:
    return clamp_range(value, 0.0, 100.0, default)


def clamp_zl_border_width(value: Any, default: int = 0) -> int:
    return max(0, min(50, clamp_int(value, default)))


def scale_position_to_fcl(value: Any) -> int:
    return max(0, min(1000, clamp_int(clamp_int(value) / 10)))


def scale_position_to_zl(value: Any) -> int:
    return max(0, min(10000, clamp_int(clamp_int(value) * 10)))


def zl_ref_to_fcl(ref: str | None) -> str:
    return "SCREEN_HEIGHT" if ref == "screen_height" else "SCREEN_WIDTH"


def fcl_ref_to_zl(ref: str | None) -> str:
    return "screen_height" if ref == "SCREEN_HEIGHT" else "screen_width"


def visibility_zl_to_fcl(value: str | None) -> str:
    return {
        "always": "ALWAYS",
        "in_game": "IN_GAME",
        "menu": "MENU",
        "in_menu": "MENU",
    }.get(value or "always", "ALWAYS")


def visibility_fcl_to_zl(value: str | None) -> str:
    return {
        "ALWAYS": "always",
        "IN_GAME": "in_game",
        "MENU": "in_menu",
    }.get(value or "ALWAYS", "always")


def default_fcl_style(name: str = "Default") -> dict[str, Any]:
    return {
        "name": name,
        "textColor": -1,
        "textSize": 12,
        "strokeColor": -12303292,
        "strokeWidth": 10,
        "cornerRadius": 100,
        "fillColor": 0,
        "textColorPressed": -1,
        "textSizePressed": 12,
        "strokeColorPressed": -12303292,
        "strokeWidthPressed": 10,
        "cornerRadiusPressed": 100,
        "fillColorPressed": -3355444,
    }


def default_zl_fallback_fcl_style(name: str = "ZL Native Default") -> dict[str, Any]:
    # Matches ZL DefaultButtonStyleConfig in ButtonStyle.kt: black 50% bg,
    # white text, no border, square corners, gray 70% pressed bg.
    return {
        "name": name,
        "textColor": -1,
        "textSize": 14,
        "strokeColor": -1,
        "strokeWidth": 0,
        "cornerRadius": 0,
        "fillColor": -2147483648,
        "textColorPressed": -1,
        "textSizePressed": 14,
        "strokeColorPressed": -1,
        "strokeWidthPressed": 0,
        "cornerRadiusPressed": 0,
        "fillColorPressed": -1282897784,
    }


def default_fcl_direction_style() -> dict[str, Any]:
    return {
        "name": "Default",
        "styleType": "BUTTON",
        "buttonStyle": {
            "interval": 50,
            "textColor": -1,
            "textSize": 12,
            "strokeColor": -12303292,
            "strokeWidth": 10,
            "cornerRadius": 100,
            "fillColor": 0,
            "textColorPressed": -1,
            "textSizePressed": 12,
            "strokeColorPressed": -12303292,
            "strokeWidthPressed": 10,
            "cornerRadiusPressed": 100,
            "fillColorPressed": -3355444,
        },
        "rockerStyle": {
            "rockerSize": 400,
            "bgCornerRadius": 500,
            "bgStrokeWidth": 20,
            "bgStrokeColor": -12303292,
            "bgFillColor": 0,
            "rockerCornerRadius": 500,
            "rockerStrokeWidth": 10,
            "rockerStrokeColor": -12303292,
            "rockerFillColor": -7829368,
        },
    }


def empty_fcl_event() -> dict[str, Any]:
    return {
        "autoKeep": False,
        "autoClick": False,
        "openMenu": False,
        "switchTouchMode": False,
        "switchMouseMode": False,
        "input": False,
        "quickInput": False,
        "outputText": "",
        "outputKeycodes": [],
        "bindViewGroup": [],
    }


def fcl_button_event() -> dict[str, Any]:
    return {
        "pointerFollow": False,
        "Movable": False,
        "pressEvent": empty_fcl_event(),
        "longPressEvent": empty_fcl_event(),
        "clickEvent": empty_fcl_event(),
        "doubleClickEvent": empty_fcl_event(),
    }


def zl_shape_to_fcl_radius(shape: dict[str, Any] | None) -> int:
    if not isinstance(shape, dict):
        return 100
    values = [clamp_zl_shape(shape.get(k), 0.0) for k in ("topStart", "topEnd", "bottomEnd", "bottomStart")]
    # FCL's ControlButton uses GradientDrawable.setCornerRadius(), so it can only
    # represent one radius for all four corners. Preserve the total amount of
    # rounding: one rounded ZL corner at 40dp becomes a uniform 10dp radius in FCL,
    # two rounded corners become 20dp, and four rounded corners stay 40dp.
    return clamp_int(sum(values) / len(values) * 10, 100)


def signed_int32(value: int) -> int:
    value &= 0xFFFFFFFF
    return value - 0x100000000 if value >= 0x80000000 else value


def apply_argb_alpha(color: int, alpha: Any) -> int:
    alpha_value = clamp_range(alpha, 0.0, 1.0, 1.0)
    if alpha_value >= 0.999:
        return color
    argb = color & 0xFFFFFFFF
    a = (argb >> 24) & 0xFF
    a = max(0, min(255, round(a * alpha_value)))
    return signed_int32((a << 24) | (argb & 0x00FFFFFF))


def zl_color_to_fcl(color: Any, fallback: int, alpha: Any = 1.0) -> int:
    # ZL serializes androidx.compose.ui.graphics.Color.value as a signed Long.
    # In the sRGB Color packing used by these layouts, Android ARGB occupies the
    # high 32 bits. Example: Long.MIN_VALUE is 0x80000000_00000000, i.e. 50%
    # black, not transparent.
    if isinstance(color, int):
        packed = color & 0xFFFFFFFFFFFFFFFF
        argb = (packed >> 32) & 0xFFFFFFFF
        if argb or color == 0:
            return apply_argb_alpha(signed_int32(argb), alpha)
        if -2147483648 <= color <= 2147483647:
            return apply_argb_alpha(color, alpha)
    return apply_argb_alpha(fallback, alpha)


def fcl_argb_to_zl_color(color: Any, fallback: int = 0) -> int:
    # Compose Color packed sRGB format: ARGB occupies the high 32 bits.
    value = clamp_int(color, fallback) & 0xFFFFFFFF
    packed = value << 32
    return packed - (1 << 64) if packed >= (1 << 63) else packed


def fcl_font_to_zl(value: Any, default: int = 12) -> int:
    return max(2, min(30, clamp_int(value, default)))


def fcl_radius_to_zl_percent(value: Any, default: int = 500) -> int:
    return max(0, min(50, clamp_int(value, default) // 10))


def fcl_ratio_to_zl(value: Any, default: int = 500) -> float:
    return max(0.0, min(1.0, clamp_int(value, default) / 1000.0))


def default_zl_joystick_style_config() -> dict[str, Any]:
    return {
        "alpha": 1.0,
        "backgroundColor": fcl_argb_to_zl_color(0x80000000),
        "joystickColor": fcl_argb_to_zl_color(0x80FFFFFF),
        "joystickCanLockColor": fcl_argb_to_zl_color(0x80FFFF00),
        "joystickLockedColor": fcl_argb_to_zl_color(0x8000FF00),
        "lockMarkColor": fcl_argb_to_zl_color(0xFFFFFFFF),
        "borderWidthRatio": 0,
        "borderColor": fcl_argb_to_zl_color(0xFFFFFFFF),
        "backgroundShape": 50,
        "joystickShape": 50,
        "joystickSize": 0.5,
    }


def fcl_rocker_style_to_zl_joystick(style: dict[str, Any]) -> dict[str, Any]:
    rocker = (style or {}).get("rockerStyle") or {}
    config = default_zl_joystick_style_config()
    config.update({
        "backgroundColor": fcl_argb_to_zl_color(rocker.get("bgFillColor", 0x80000000)),
        "joystickColor": fcl_argb_to_zl_color(rocker.get("rockerFillColor", 0x80FFFFFF)),
        "borderColor": fcl_argb_to_zl_color(rocker.get("bgStrokeColor", 0xFFFFFFFF)),
        "borderWidthRatio": max(0, min(50, clamp_int(rocker.get("bgStrokeWidth", 0)) // 10)),
        "backgroundShape": fcl_radius_to_zl_percent(rocker.get("bgCornerRadius", 500)),
        "joystickShape": fcl_radius_to_zl_percent(rocker.get("rockerCornerRadius", 500)),
        "joystickSize": fcl_ratio_to_zl(rocker.get("rockerSize", 500)),
    })
    return {
        "uuid": short_id(),
        "lightStyle": config,
        "darkStyle": dict(config),
    }


def direction_style_map(styles: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    return {str(style.get("name")): style for style in styles or [] if isinstance(style, dict)}


def resolve_direction_style(direction: dict[str, Any], styles: dict[str, dict[str, Any]]) -> dict[str, Any]:
    style = direction.get("style")
    if isinstance(style, dict):
        return style
    return styles.get(str(style), {})


def style_name_for_zl_style(base_name: str, uuid_value: str) -> str:
    suffix = uuid_value[:6] if uuid_value else short_id()[:6]
    return f"ZL {base_name} {suffix}"


def zl_styles_to_fcl(styles: list[dict[str, Any]]) -> tuple[list[dict[str, Any]], dict[str, str]]:
    result: list[dict[str, Any]] = []
    mapping: dict[str, str] = {}
    used: set[str] = set()

    for style in styles or []:
        uuid_value = str(style.get("uuid", ""))
        base_name = str(style.get("name") or uuid_value or "Style")
        name = style_name_for_zl_style(base_name, uuid_value)
        suffix = 2
        while name in used:
            name = f"{style_name_for_zl_style(base_name, uuid_value)}_{suffix}"
            suffix += 1
        used.add(name)
        if uuid_value:
            mapping[uuid_value] = name

        light = style.get("lightStyle") or {}
        result.append({
            "name": name,
            "textColor": zl_color_to_fcl(light.get("contentColor"), -1),
            "textSize": clamp_int(light.get("fontSize"), 12),
            "strokeColor": zl_color_to_fcl(light.get("borderColor"), -12303292),
            "strokeWidth": clamp_int(light.get("borderWidth"), 1) * 10,
            "cornerRadius": zl_shape_to_fcl_radius(light.get("borderRadius")),
            "fillColor": zl_color_to_fcl(light.get("backgroundColor"), 0, light.get("alpha", 1.0)),
            "textColorPressed": zl_color_to_fcl(light.get("pressedContentColor"), -1),
            "textSizePressed": clamp_int(light.get("pressedFontSize"), clamp_int(light.get("fontSize"), 12)),
            "strokeColorPressed": zl_color_to_fcl(light.get("pressedBorderColor"), -12303292),
            "strokeWidthPressed": clamp_int(light.get("pressedBorderWidth"), clamp_int(light.get("borderWidth"), 1)) * 10,
            "cornerRadiusPressed": zl_shape_to_fcl_radius(light.get("pressedBorderRadius")),
            "fillColorPressed": zl_color_to_fcl(light.get("pressedBackgroundColor"), -3355444, light.get("pressedAlpha", 1.0)),
        })

    if not result:
        result.append(default_zl_fallback_fcl_style())
    elif "ZL Native Default" not in {s["name"] for s in result}:
        result.insert(0, default_zl_fallback_fcl_style())
    return result, mapping


def fcl_styles_to_zl(styles: list[dict[str, Any]]) -> tuple[list[dict[str, Any]], dict[str, str]]:
    result: list[dict[str, Any]] = []
    mapping: dict[str, str] = {}
    for style in styles or [default_fcl_style()]:
        name = str(style.get("name") or "Default")
        sid = short_id()
        mapping[name] = sid
        radius = clamp_zl_shape(clamp_float(style.get("cornerRadius", 0), 0.0) / 10.0)
        pressed_radius = clamp_zl_shape(clamp_float(style.get("cornerRadiusPressed", style.get("cornerRadius", 0)), 0.0) / 10.0)
        light = {
            "alpha": 1.0,
            "pressedAlpha": 1.0,
            "backgroundColor": fcl_argb_to_zl_color(style.get("fillColor", 0)),
            "pressedBackgroundColor": fcl_argb_to_zl_color(style.get("fillColorPressed", -3355444)),
            "contentColor": fcl_argb_to_zl_color(style.get("textColor", -1), -1),
            "pressedContentColor": fcl_argb_to_zl_color(style.get("textColorPressed", -1), -1),
            "fontSize": fcl_font_to_zl(style.get("textSize", 12)),
            "pressedFontSize": fcl_font_to_zl(style.get("textSizePressed", style.get("textSize", 12))),
            "borderWidth": clamp_zl_border_width(clamp_int(style.get("strokeWidth"), 10) // 10),
            "pressedBorderWidth": clamp_zl_border_width(clamp_int(style.get("strokeWidthPressed"), 10) // 10),
            "borderColor": fcl_argb_to_zl_color(style.get("strokeColor", -12303292), -12303292),
            "pressedBorderColor": fcl_argb_to_zl_color(style.get("strokeColorPressed", -12303292), -12303292),
            "borderRadius": {"topStart": radius, "topEnd": radius, "bottomEnd": radius, "bottomStart": radius},
            "pressedBorderRadius": {"topStart": pressed_radius, "topEnd": pressed_radius, "bottomEnd": pressed_radius, "bottomStart": pressed_radius},
        }
        result.append({
            "name": name,
            "uuid": sid,
            "animateSwap": False,
            "commonStyle": True,
            "lightStyle": light,
            "darkStyle": dict(light),
        })
    return result, mapping


def normalize_zl_key(event_key: str) -> str:
    key = str(event_key or "").strip()
    upper_key = key.upper()
    if upper_key.startswith("GLFW_") or upper_key.startswith("MOUSE_"):
        key = upper_key
    return ZL_KEY_ALIASES.get(key, key)


def convert_key_to_fcl(event_key: str, strict: bool, substitutions: list[dict[str, Any]] | None = None) -> int | None:
    event_key = normalize_zl_key(event_key)
    if event_key in FCL_MOUSE:
        return FCL_MOUSE[event_key]
    if event_key in GLFW_TO_FCL:
        return GLFW_TO_FCL[event_key]
    if event_key in ZL_TO_FCL_FALLBACKS:
        keycode, reason = ZL_TO_FCL_FALLBACKS[event_key]
        warn(f"ZL key event {event_key!r} has no exact FCL equivalent; {reason}", strict)
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "key", "key": event_key},
                {"type": "fcl_keycode", "keycode": keycode},
                reason,
                category="keys",
            ))
        return keycode
    if event_key in UNSUPPORTED_ZL_KEY_REASONS:
        warn(f"ZL key event {event_key!r} has no FCL control keycode equivalent: {UNSUPPORTED_ZL_KEY_REASONS[event_key]}; substituted with UNKNOWN", strict)
    else:
        warn(f"unsupported ZL key event {event_key!r}; substituted with UNKNOWN", strict)
    fallback = GLFW_TO_FCL["GLFW_KEY_UNKNOWN"]
    if substitutions is not None:
        substitutions.append(substitution(
            {"type": "key", "key": event_key},
            {"type": "fcl_keycode", "keycode": fallback},
            "No known FCL equivalent; substituted with UNKNOWN",
            category="keys",
        ))
    return fallback


def convert_key_to_zl(
    keycode: int,
    strict: bool,
    auto_click: bool = False,
    label: str = "",
    substitutions: list[dict[str, Any]] | None = None,
) -> tuple[str, str] | None:
    if keycode == -1 and label.strip() == "*":
        return "key", "GLFW_KEY_KP_MULTIPLY"
    if keycode in FCL_MOUSE_REVERSE:
        return "key", FCL_MOUSE_REVERSE[keycode]
    if keycode in FCL_SCROLL_REVERSE:
        single_event, long_event = FCL_SCROLL_REVERSE[keycode]
        return "launcher_event", long_event if auto_click else single_event
    if keycode in FCL_TO_GLFW:
        return "key", FCL_TO_GLFW[keycode]
    if keycode in FCL_TO_ZL_FALLBACKS:
        (etype, key), reason = FCL_TO_ZL_FALLBACKS[keycode]
        warn(f"FCL keycode {keycode!r} has no exact ZL equivalent; {reason}", strict)
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_keycode", "keycode": keycode},
                {"type": etype, "key": key},
                reason,
                category="keys",
            ))
        return etype, key
    if keycode in UNSUPPORTED_FCL_KEY_REASONS:
        warn(f"FCL keycode {keycode!r} has no ZL control event equivalent: {UNSUPPORTED_FCL_KEY_REASONS[keycode]}; substituted with GLFW_KEY_UNKNOWN", strict)
    else:
        warn(f"unsupported FCL keycode {keycode!r}; substituted with GLFW_KEY_UNKNOWN", strict)
    if substitutions is not None:
        substitutions.append(substitution(
            {"type": "fcl_keycode", "keycode": keycode},
            {"type": "key", "key": "GLFW_KEY_UNKNOWN"},
            "No known ZL equivalent; substituted with GLFW_KEY_UNKNOWN",
            category="keys",
        ))
    return "key", "GLFW_KEY_UNKNOWN"


def fcl_keycode_list(value: Any) -> list[Any]:
    if isinstance(value, list):
        return value
    if value is None:
        return []
    return [value]


def estimate_wrap_content_dp(widget: dict[str, Any], style_name: str | None, fcl_styles: list[dict[str, Any]]) -> tuple[int, int]:
    text = text_default(widget.get("text"))
    style = next((item for item in fcl_styles if item.get("name") == style_name), None) or default_zl_fallback_fcl_style()
    font_size = max(2, clamp_int(style.get("textSize"), 14))
    lines = text.splitlines() or [""]
    longest = max((len(line) for line in lines), default=0)
    width = max(5, min(480, round(longest * font_size * 0.62 + 8)))
    height = max(5, min(240, round(len(lines) * font_size * 1.25 + 6)))
    return width, height


def make_base_info_from_zl(button: dict[str, Any], layer_visibility: str, strict: bool = False, label: str = "", style_name: str | None = None, fcl_styles: list[dict[str, Any]] | None = None) -> dict[str, Any]:
    size = button.get("buttonSize") or {}
    size_kind = size.get("type")
    if size_kind in ("absolute", "dp"):
        size_type = "ABSOLUTE"
        absolute_width = clamp_int(size.get("widthDp"), 50)
        absolute_height = clamp_int(size.get("heightDp"), 50)
    elif size_kind == "wrap_content":
        size_type = "ABSOLUTE"
        absolute_width, absolute_height = estimate_wrap_content_dp(button, style_name, fcl_styles or [])
        widget_label = label or text_default(button.get("text")) or str(button.get("uuid") or "") or "<unnamed>"
        warn(f"ZL wrap_content size on widget {widget_label!r} has no exact FCL equivalent; estimated dp size", strict, once=True)
    else:
        size_type = "PERCENTAGE"
        absolute_width = clamp_int(size.get("widthDp"), 50)
        absolute_height = clamp_int(size.get("heightDp"), 50)
    visibility = visibility_zl_to_fcl(button.get("visibilityType") or layer_visibility)
    return {
        "visibilityType": visibility,
        "xPosition": scale_position_to_fcl((button.get("position") or {}).get("x", 0)),
        "yPosition": scale_position_to_fcl((button.get("position") or {}).get("y", 0)),
        "sizeType": size_type,
        "absoluteWidth": absolute_width,
        "absoluteHeight": absolute_height,
        "percentageWidth": {
            "reference": zl_ref_to_fcl(size.get("widthReference")),
            "size": scale_position_to_fcl(size.get("widthPercentage", 500)),
        },
        "percentageHeight": {
            "reference": zl_ref_to_fcl(size.get("heightReference")),
            "size": scale_position_to_fcl(size.get("heightPercentage", 500)),
        },
    }


def warn_unmapped_layer_flags(layer: dict[str, Any], strict: bool) -> None:
    layer_name = str(layer.get("name") or layer.get("uuid") or "Layer")
    if layer.get("hideWhenMouse"):
        warn(f"ZL layer {layer_name!r} hideWhenMouse has no FCL equivalent; skipped", strict, once=True)
    if layer.get("hideWhenGamepad"):
        warn(f"ZL layer {layer_name!r} hideWhenGamepad has no FCL equivalent; skipped", strict, once=True)
    if layer.get("hideWhenJoystick"):
        warn(f"ZL layer {layer_name!r} hideWhenJoystick has no FCL equivalent; skipped", strict, once=True)


def normalize_zl_click_events(events: list[dict[str, str]]) -> list[dict[str, str]]:
    deduped = dedupe_events(events)
    send_text_events = [event for event in deduped if event.get("type") == "send_text" and event.get("key")]
    other_events = [event for event in deduped if event.get("type") != "send_text"]
    if send_text_events:
        other_events.append(send_text_events[0])
    return other_events


def normalize_zl_layout(layout: dict[str, Any]) -> dict[str, Any]:
    """Fill fields required by current ZL kotlinx models without changing semantics."""
    result = deep_copy_json(layout)
    result.setdefault("special", {})
    for layer in result.get("layers") or []:
        if not isinstance(layer, dict):
            continue
        layer.setdefault("hideWhenMouse", True)
        layer.setdefault("hideWhenGamepad", True)
        layer.setdefault("hideWhenJoystick", False)
        layer.setdefault("normalButtons", [])
        layer.setdefault("textBoxes", [])
    return result


def strip_converter_meta(value: Any) -> Any:
    if isinstance(value, dict):
        return {
            key: strip_converter_meta(item)
            for key, item in value.items()
            if key != META_KEY
        }
    if isinstance(value, list):
        return [strip_converter_meta(item) for item in value]
    return value


def fcl_size_to_zl(value: Any) -> int:
    # FCL PercentageSize.size is a per-mille-like value used directly as size / 1000f.
    # ZL buttonSize percentage uses a 100..10000 scale. So FCL 140 means 14%, ZL 1400.
    return max(100, min(10000, clamp_int(clamp_int(value, 50) * 10)))


def make_zl_button_size(base_info: dict[str, Any], absolute_as_percentage: bool = False, aspect: float = 16 / 9) -> dict[str, Any]:
    pw = base_info.get("percentageWidth") or {}
    ph = base_info.get("percentageHeight") or {}
    if base_info.get("sizeType") == "ABSOLUTE" and absolute_as_percentage:
        # FCL absolute sizes are Android dp, but many shared FCL layouts were tuned
        # visually for one target screen. ZL percentage sizing preserves that visual
        # proportion better across devices when the target aspect ratio is known.
        screen_height_dp = 411.0
        screen_width_dp = screen_height_dp * max(0.1, clamp_float(aspect, 16 / 9))
        width_percentage = max(100, min(10000, round(clamp_zl_dp(base_info.get("absoluteWidth", 50)) / screen_width_dp * 10000)))
        height_percentage = max(100, min(10000, round(clamp_zl_dp(base_info.get("absoluteHeight", 50)) / screen_height_dp * 10000)))
        return {
            "type": "percentage",
            "widthDp": clamp_zl_dp(base_info.get("absoluteWidth", 50)),
            "heightDp": clamp_zl_dp(base_info.get("absoluteHeight", 50)),
            "widthPercentage": width_percentage,
            "heightPercentage": height_percentage,
            "widthReference": "screen_width",
            "heightReference": "screen_height",
        }
    return {
        "type": "dp" if base_info.get("sizeType") == "ABSOLUTE" else "percentage",
        "widthDp": clamp_zl_dp(base_info.get("absoluteWidth", 50)),
        "heightDp": clamp_zl_dp(base_info.get("absoluteHeight", 50)),
        "widthPercentage": fcl_size_to_zl(pw.get("size", 50)),
        "heightPercentage": fcl_size_to_zl(ph.get("size", 50)),
        "widthReference": fcl_ref_to_zl(pw.get("reference")),
        "heightReference": fcl_ref_to_zl(ph.get("reference")),
    }


def apply_zl_event_to_fcl(
    event: dict[str, Any],
    fcl_event: dict[str, Any],
    strict: bool,
    substitutions: list[dict[str, Any]] | None = None,
    layer_id_map: dict[str, str] | None = None,
) -> None:
    etype = event.get("type")
    raw_key = str(event.get("key", ""))
    key = normalize_zl_key(raw_key)
    press_event = fcl_event["pressEvent"]
    click_event = fcl_event["clickEvent"]

    if etype == "key":
        keycode = convert_key_to_fcl(key, strict, substitutions=substitutions)
        if keycode is not None:
            press_event["outputKeycodes"].append(keycode)
    elif etype == "launcher_event":
        if key in FCL_MOUSE:
            press_event["outputKeycodes"].append(FCL_MOUSE[key])
        elif key == "launcher.event.switch_ime":
            click_event["input"] = True
        elif key == "launcher.event.switch_menu":
            click_event["openMenu"] = True
        elif key == "launcher.event.scroll_up.single":
            click_event["outputKeycodes"].append(1003)
        elif key == "launcher.event.scroll_down.single":
            click_event["outputKeycodes"].append(1004)
        elif key in ("launcher.event.scroll_up", "launcher.event.scroll_down"):
            code = 1003 if key.endswith("scroll_up") else 1004
            press_event["autoClick"] = True
            press_event["outputKeycodes"].append(code)
        else:
            keycode = convert_key_to_fcl(key, strict, substitutions=substitutions)
            if keycode is not None:
                press_event["outputKeycodes"].append(keycode)
    elif etype in ("switch_layer", "show_layer", "hide_layer"):
        # Layer events are handled in apply_zl_layer_events_to_fcl() so show/hide
        # can be converted with state awareness instead of blind toggles.
        return
    elif etype == "send_text":
        click_event["outputText"] = raw_key
    elif etype is None:
        return
    else:
        reason = f"unsupported ZL event type {etype!r}; substituted with no-op text event"
        warn(reason, strict)
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": str(etype), "key": raw_key},
                {"type": "send_text", "key": ""},
                reason,
                category="events",
            ))
        click_event["outputText"] = click_event.get("outputText", "")


def apply_zl_layer_events_to_fcl(
    events: list[dict[str, Any]],
    fcl_event: dict[str, Any],
    strict: bool,
    initial_layer_state: dict[str, bool],
    layer_id_map: dict[str, str] | None = None,
    substitutions: list[dict[str, Any]] | None = None,
) -> None:
    local_state = dict(initial_layer_state or {})
    toggles: list[str] = []

    for event in events:
        etype = event.get("type")
        if etype not in ("switch_layer", "show_layer", "hide_layer"):
            continue
        raw_key = str(event.get("key", ""))
        target_id = (layer_id_map or {}).get(raw_key) or raw_key
        if not target_id:
            continue

        current = bool(local_state.get(target_id, False))
        should_toggle = False
        if etype == "switch_layer":
            should_toggle = True
            local_state[target_id] = not current
        elif etype == "show_layer":
            if not current:
                should_toggle = True
                local_state[target_id] = True
        elif etype == "hide_layer":
            if current:
                should_toggle = True
                local_state[target_id] = False

        if should_toggle:
            toggles.append(target_id)
        elif etype != "switch_layer" and substitutions is not None:
            substitutions.append(substitution(
                {"type": etype, "key": raw_key},
                {"type": "no_op", "key": target_id},
                f"Layer already {'visible' if etype == 'show_layer' else 'hidden'} in the simulated ZL state; skipped FCL toggle",
                category="layers",
            ))

    # In FCL every bindViewGroup entry toggles. If the same layer would be toggled
    # twice by one ZL button, the two toggles cancel out, so only emit odd counts.
    counts = Counter(toggles)
    ordered_unique: list[str] = []
    seen: set[str] = set()
    for target_id in toggles:
        if target_id in seen or counts[target_id] % 2 == 0:
            continue
        seen.add(target_id)
        ordered_unique.append(target_id)

    fcl_event["clickEvent"]["bindViewGroup"].extend(ordered_unique)

    if substitutions is not None and any(event.get("type") in ("show_layer", "hide_layer") for event in events):
        substitutions.append(substitution(
            {"type": "zl_layer_state_events", "events": [deep_copy_json(e) for e in events if e.get("type") in ("switch_layer", "show_layer", "hide_layer")]},
            {"type": "fcl_bindViewGroup", "keys": ordered_unique},
            "Converted ZL show/hide/switch layer events by simulating initial layer visibility and emitting only necessary FCL toggles",
            category="layers",
        ))


def resolve_zl_button_style_name(style_uuid: Any, style_map: dict[str, str], fallback: str = "ZL Native Default") -> str:
    if style_uuid is None:
        return fallback
    return style_map.get(str(style_uuid), fallback)


def overlay_shared_fields_fcl(original: dict[str, Any], current: dict[str, Any], layer_visibility: str, style_map: dict[str, str], strict: bool, fcl_styles: list[dict[str, Any]] | None = None) -> dict[str, Any]:
    restored = deep_copy_json(original)
    style_uuid = current.get("buttonStyle")
    style_name = resolve_zl_button_style_name(style_uuid, style_map, restored.get("style", "ZL Native Default"))
    restored["id"] = str(current.get("uuid") or restored.get("id") or fcl_id())
    restored["text"] = text_default(current.get("text"))
    restored["style"] = style_name
    restored["baseInfo"] = make_base_info_from_zl(current, layer_visibility, strict=strict, label=text_default(current.get("text")), style_name=style_name, fcl_styles=fcl_styles)
    return restored


def zl_button_to_fcl(
    button: dict[str, Any],
    layer_visibility: str,
    style_map: dict[str, str],
    strict: bool,
    layer_id_map: dict[str, str] | None = None,
    fcl_styles: list[dict[str, Any]] | None = None,
    initial_layer_state: dict[str, bool] | None = None,
    current_layer_id: str | None = None,
) -> dict[str, Any]:
    original = meta_original(button, "fcl", "button")
    if original is not None:
        restored = overlay_shared_fields_fcl(original, button, layer_visibility, style_map, strict, fcl_styles=fcl_styles)
        return set_meta(restored, make_meta("zl", "button", str(button.get("uuid") or restored.get("id") or fcl_id()), button))

    direction_original = meta_original(button, "fcl", "direction")
    if direction_original is not None:
        return None

    event = fcl_button_event()
    substitutions: list[dict[str, Any]] = []
    click_events = button.get("clickEvents") or []
    for click_event in click_events:
        apply_zl_event_to_fcl(click_event, event, strict, substitutions=substitutions, layer_id_map=layer_id_map)
    simulated_state = dict(initial_layer_state or {})
    if current_layer_id:
        # A button can only be pressed while its own ZL layer is visible. Hidden
        # panels like settings are opened first, then their close buttons run with
        # that panel visible, not with the layout's initial hidden state.
        simulated_state[current_layer_id] = True
    apply_zl_layer_events_to_fcl(
        click_events,
        event,
        strict,
        initial_layer_state=simulated_state,
        layer_id_map=layer_id_map,
        substitutions=substitutions,
    )

    if button.get("isToggleable"):
        event["pressEvent"]["autoKeep"] = True

    style_uuid = button.get("buttonStyle")
    style_name = resolve_zl_button_style_name(style_uuid, style_map)
    result = {
        "id": str(button.get("uuid") or fcl_id()),
        "text": text_default(button.get("text")),
        "style": style_name,
        "baseInfo": make_base_info_from_zl(button, layer_visibility, strict=strict, label=text_default(button.get("text")), style_name=style_name, fcl_styles=fcl_styles),
        "event": event,
    }
    return set_meta(result, make_meta("zl", "button", str(button.get("uuid") or result["id"]), button, append_substitutions(None, substitutions)))


def zl_textbox_to_fcl(textbox: dict[str, Any], layer_visibility: str, style_map: dict[str, str], strict: bool, fcl_styles: list[dict[str, Any]] | None = None) -> dict[str, Any]:
    original = meta_original(textbox, "fcl", "button")
    if original is not None:
        restored = overlay_shared_fields_fcl(original, textbox, layer_visibility, style_map, strict, fcl_styles=fcl_styles)
        return set_meta(restored, make_meta("zl", "textbox", str(textbox.get("uuid") or restored.get("id") or fcl_id()), textbox))

    direction_original = meta_original(textbox, "fcl", "direction")
    if direction_original is not None:
        return None

    style_uuid = textbox.get("buttonStyle")
    style_name = resolve_zl_button_style_name(style_uuid, style_map)
    result = {
        "id": str(textbox.get("uuid") or fcl_id()),
        "text": text_default(textbox.get("text")),
        "style": style_name,
        "baseInfo": make_base_info_from_zl(textbox, layer_visibility, strict=strict, label=text_default(textbox.get("text")), style_name=style_name, fcl_styles=fcl_styles),
        "event": fcl_button_event(),
    }
    return set_meta(result, make_meta("zl", "textbox", str(textbox.get("uuid") or result["id"]), textbox))


def fcl_button_is_decorative(button: dict[str, Any]) -> bool:
    if fcl_button_has_payload(button):
        return False
    event_root = button.get("event") or {}
    if event_root.get("pointerFollow") or event_root.get("Movable"):
        return False
    return True


def order_fcl_buttons_for_layer(buttons: list[dict[str, Any]]) -> list[dict[str, Any]]:
    # FCL adds Android views in buttonList order; later views are visually and
    # interactively above earlier ones. Keep decorative/text-only widgets first so
    # actual buttons float above them.
    decorated = [(index, button) for index, button in enumerate(buttons)]
    decorated.sort(key=lambda item: (0 if fcl_button_is_decorative(item[1]) else 1, item[0]))
    return [button for _, button in decorated]


def layer_is_background_like(group: dict[str, Any]) -> bool:
    buttons = ((group.get("viewData") or {}).get("buttonList") or [])
    if not buttons:
        return False
    decorative = sum(1 for button in buttons if fcl_button_is_decorative(button))
    return decorative == len(buttons)


def order_fcl_view_groups(groups: list[dict[str, Any]]) -> list[dict[str, Any]]:
    # FCL adds groups in order; later groups are on top. Put background/decorative
    # groups lower so panels/buttons remain clickable above them.
    decorated = [(index, group) for index, group in enumerate(groups)]
    decorated.sort(key=lambda item: (0 if layer_is_background_like(item[1]) else 1, item[0]))
    return [group for _, group in decorated]


def infer_visible_companion_layers(data: dict[str, Any], layer_id_map: dict[str, str]) -> dict[str, set[str]]:
    companions: dict[str, set[str]] = {}
    layer_ids = {str(layer.get("uuid") or "") for layer in data.get("layers") or [] if layer.get("uuid")}
    hidden_layers = {str(layer.get("uuid") or "") for layer in data.get("layers") or [] if layer.get("uuid") and layer.get("hide")}
    opener_targets: dict[str, set[str]] = defaultdict(set)

    for layer in data.get("layers") or []:
        source_id = str(layer.get("uuid") or "")
        for button in layer.get("normalButtons") or []:
            events = button.get("clickEvents") or []
            visible_targets: list[str] = []
            for event in events:
                etype = event.get("type")
                raw_key = str(event.get("key", ""))
                target_id = layer_id_map.get(raw_key) or raw_key
                if etype not in ("show_layer", "switch_layer") or target_id not in layer_ids:
                    continue
                visible_targets.append(target_id)
                if source_id:
                    opener_targets[target_id].add(source_id)
            if len(visible_targets) < 2:
                continue
            group = set(visible_targets)
            for target_id in group:
                companions.setdefault(target_id, set()).update(group)

    # ZL layouts often split one panel into a decorative frame layer and a button
    # layer. A button outside that panel opens both at once, while buttons inside
    # one panel layer hide both. FCL only toggles groups, so treat the opener layer
    # as visible while converting hide_layer events for all co-opened hidden layers.
    for target_id, source_ids in opener_targets.items():
        if target_id not in hidden_layers:
            continue
        companion_ids = companions.get(target_id, {target_id})
        co_opened = any(companion_id in opener_targets for companion_id in companion_ids)
        if not co_opened:
            continue
        for source_id in source_ids:
            if source_id and source_id not in companion_ids and source_id in hidden_layers:
                companions.setdefault(target_id, set()).add(source_id)

    return companions


def zl_to_fcl(data: dict[str, Any], strict: bool = False) -> dict[str, Any]:
    root_original = meta_original(data, "fcl", "controller")
    info = data.get("info") or {}
    styles, style_map = zl_styles_to_fcl(data.get("styles") or [])
    view_groups = []

    layer_id_map: dict[str, str] = {}
    initial_layer_state: dict[str, bool] = {}
    for layer in data.get("layers") or []:
        layer_original = meta_original(layer, "fcl", "viewGroup")
        restored_group = deep_copy_json(layer_original) if layer_original is not None else {}
        layer_uuid = str(layer.get("uuid") or restored_group.get("id") or fcl_id())
        layer_id_map[str(layer.get("uuid") or layer_uuid)] = layer_uuid
        initial_layer_state[layer_uuid] = not bool(layer.get("hide"))
    companion_layers = infer_visible_companion_layers(data, layer_id_map)

    for layer in data.get("layers") or []:
        layer_original = meta_original(layer, "fcl", "viewGroup")
        warn_unmapped_layer_flags(layer, strict)
        layer_visibility = layer.get("visibilityType", "always")
        current_layer_id = layer_id_map.get(str(layer.get("uuid") or ""), str(layer.get("uuid") or ""))
        layer_state_for_buttons = dict(initial_layer_state)
        for companion_id in companion_layers.get(current_layer_id, {current_layer_id}):
            layer_state_for_buttons[companion_id] = True
        buttons = [
            converted for converted in
            (zl_button_to_fcl(button, layer_visibility, style_map, strict, layer_id_map=layer_id_map, fcl_styles=styles, initial_layer_state=layer_state_for_buttons, current_layer_id=current_layer_id) for button in layer.get("normalButtons") or [])
            if converted is not None
        ]
        buttons.extend(
            converted for converted in
            (zl_textbox_to_fcl(textbox, layer_visibility, style_map, strict, fcl_styles=styles) for textbox in layer.get("textBoxes") or [])
            if converted is not None
        )
        restored_group = deep_copy_json(layer_original) if layer_original is not None else {}
        direction_list = (((restored_group.get("viewData") or {}).get("directionList")) if isinstance(restored_group.get("viewData"), dict) else None) or []
        result_group = {
            "id": str(layer.get("uuid") or restored_group.get("id") or fcl_id()),
            "name": str(layer.get("name") or restored_group.get("name") or "Layer"),
            "visibility": "INVISIBLE" if layer.get("hide") else "VISIBLE",
            "viewData": {
                "buttonList": order_fcl_buttons_for_layer(buttons),
                "directionList": deep_copy_json(direction_list),
            },
        }
        result_group = set_meta(result_group, make_meta("zl", "layer", str(layer.get("uuid") or result_group["id"]), layer))
        meta = get_meta(result_group)
        if meta and direction_list:
            meta.setdefault("original", {})
            if isinstance(meta["original"], dict):
                meta["original"]["directionList"] = deep_copy_json(direction_list)
        view_groups.append(result_group)

    result = deep_copy_json(root_original) if root_original is not None else {}
    result.update({
        "id": str(data.get("id") or result.get("id") or short_id()[:8]),
        "name": text_default(info.get("name")) or str(result.get("name") or "Converted from Zalith"),
        "version": text_default(info.get("versionName")) or str(result.get("version") or "1.0"),
        "versionCode": clamp_int(info.get("versionCode"), clamp_int(result.get("versionCode"), 1)),
        "author": text_default(info.get("author")) or str(result.get("author") or ""),
        "description": text_default(info.get("description")) or str(result.get("description") or ""),
        "controllerVersion": clamp_int(result.get("controllerVersion"), FCL_CONTROLLER_VERSION),
        "buttonStyles": styles,
        "directionStyles": deep_copy_json(result.get("directionStyles") or [default_fcl_direction_style()]),
        "viewGroups": order_fcl_view_groups(view_groups),
    })
    return set_meta(result, make_meta("zl", "layout", str(data.get("id") or result["id"]), data))


def fcl_event_has_payload(event: dict[str, Any]) -> bool:
    return bool(
        fcl_keycode_list(event.get("outputKeycodes"))
        or event.get("input")
        or event.get("openMenu")
        or event.get("outputText")
        or event.get("bindViewGroup")
        or event.get("switchTouchMode")
        or event.get("switchMouseMode")
        or event.get("quickInput")
    )


def fcl_event_to_zl_events(
    event: dict[str, Any],
    strict: bool,
    label: str = "",
    event_name: str = "pressEvent",
    group_ids_by_name: dict[str, str] | None = None,
    substitutions: list[dict[str, Any]] | None = None,
) -> list[dict[str, str]]:
    result: list[dict[str, str]] = []
    auto_click = bool(event.get("autoClick"))
    keycodes = [clamp_int(keycode) for keycode in fcl_keycode_list(event.get("outputKeycodes"))]
    if event_name in {"clickEvent", "doubleClickEvent"} and fcl_event_has_payload(event):
        reason = f"FCL {event_name} has no exact ZL timing equivalent; converted to a normal ZL press/release event"
        warn(f"{reason} on button {label!r}", strict, once=True)
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_event", "event": event_name},
                {"type": "zl_click_events"},
                reason,
                category="events",
            ))
    if event_name == "longPressEvent" and fcl_event_has_payload(event):
        reason = "FCL longPressEvent has no exact ZL timing equivalent; converted to a normal event"
        warn(f"{reason} on button {label!r}", strict, once=True)
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_event", "event": event_name},
                {"type": "zl_click_events"},
                reason,
                category="events",
            ))
    if auto_click and any(keycode not in FCL_SCROLL_REVERSE for keycode in keycodes):
        reason = "FCL autoClick only has a ZL equivalent for scroll events; non-scroll keys are converted as normal press events"
        warn(reason, strict, once=True)
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_auto_click", "event": event_name},
                {"type": "zl_normal_press"},
                reason,
                category="events",
            ))
    for keycode in keycodes:
        converted = convert_key_to_zl(keycode, strict, auto_click=auto_click, label=label, substitutions=substitutions)
        if converted:
            etype, key = converted
            result.append({"type": etype, "key": key})
    if event.get("input"):
        result.append({"type": "launcher_event", "key": "launcher.event.switch_ime"})
    if event.get("openMenu"):
        result.append({"type": "launcher_event", "key": "launcher.event.switch_menu"})
    if event.get("switchTouchMode"):
        reason = "FCL switchTouchMode has no ZL equivalent; substituted with launcher menu toggle"
        warn(reason, strict, once=True)
        result.append({"type": "launcher_event", "key": "launcher.event.switch_menu"})
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_event", "key": "switchTouchMode"},
                {"type": "launcher_event", "key": "launcher.event.switch_menu"},
                reason,
                category="events",
            ))
    if event.get("switchMouseMode"):
        reason = "FCL switchMouseMode has no ZL equivalent; substituted with launcher menu toggle"
        warn(reason, strict, once=True)
        result.append({"type": "launcher_event", "key": "launcher.event.switch_menu"})
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_event", "key": "switchMouseMode"},
                {"type": "launcher_event", "key": "launcher.event.switch_menu"},
                reason,
                category="events",
            ))
    if event.get("quickInput"):
        reason = "FCL quickInput has no ZL equivalent; substituted with input method toggle"
        warn(reason, strict, once=True)
        result.append({"type": "launcher_event", "key": "launcher.event.switch_ime"})
        if substitutions is not None:
            substitutions.append(substitution(
                {"type": "fcl_event", "key": "quickInput"},
                {"type": "launcher_event", "key": "launcher.event.switch_ime"},
                reason,
                category="events",
            ))
    if event.get("outputText"):
        result.append({"type": "send_text", "key": str(event.get("outputText"))})
    group_ids_by_name = group_ids_by_name or {}
    bind_groups = [str(group_id) for group_id in event.get("bindViewGroup") or []]
    suppress_chat_layer = any(
        item.get("type") == "key" and item.get("key") == "GLFW_KEY_T"
        for item in result
    ) and group_ids_by_name.get("聊天") in bind_groups
    for group_id in bind_groups:
        if suppress_chat_layer and group_id == group_ids_by_name.get("聊天"):
            continue
        result.append({"type": "switch_layer", "key": group_id})
    return result


def dedupe_events(events: list[dict[str, str]]) -> list[dict[str, str]]:
    result: list[dict[str, str]] = []
    seen: set[tuple[str, str]] = set()
    for event in events:
        key = (event.get("type", ""), event.get("key", ""))
        if key not in seen:
            seen.add(key)
            result.append(event)
    return result



def fcl_button_has_payload(button: dict[str, Any]) -> bool:
    event_root = button.get("event") or {}
    return any(
        fcl_event_has_payload(event_root.get(event_name) or {})
        for event_name in ("pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent")
    )


def fcl_button_rect(button: dict[str, Any], aspect: float) -> tuple[float, float, float, float]:
    base_info = button.get("baseInfo") or {}
    screen_h = 10000.0
    screen_w = screen_h * max(0.1, clamp_float(aspect, 16 / 9))

    if base_info.get("sizeType") == "ABSOLUTE":
        width = max(1.0, clamp_zl_dp(base_info.get("absoluteWidth", 50)) * 10.0)
        height = max(1.0, clamp_zl_dp(base_info.get("absoluteHeight", 50)) * 10.0)
    else:
        pw = base_info.get("percentageWidth") or {}
        ph = base_info.get("percentageHeight") or {}
        width_ref = screen_h if pw.get("reference") == "SCREEN_HEIGHT" else screen_w
        height_ref = screen_h if ph.get("reference") == "SCREEN_HEIGHT" else screen_w
        width = max(1.0, width_ref * clamp_int(pw.get("size", 50), 50) / 1000.0)
        height = max(1.0, height_ref * clamp_int(ph.get("size", 50), 50) / 1000.0)

    x = (screen_w - width) * clamp_int(base_info.get("xPosition", 0)) / 1000.0
    y = (screen_h - height) * clamp_int(base_info.get("yPosition", 0)) / 1000.0
    return x, y, x + width, y + height


def rect_area(rect: tuple[float, float, float, float]) -> float:
    return max(0.0, rect[2] - rect[0]) * max(0.0, rect[3] - rect[1])


def screen_area(aspect: float) -> float:
    return 10000.0 * 10000.0 * max(0.1, clamp_float(aspect, 16 / 9))


def fcl_button_area_ratio(button: dict[str, Any], aspect: float) -> float:
    return rect_area(fcl_button_rect(button, aspect)) / max(1.0, screen_area(aspect))


def rect_overlap_area(a: tuple[float, float, float, float], b: tuple[float, float, float, float]) -> float:
    return max(0.0, min(a[2], b[2]) - max(a[0], b[0])) * max(0.0, min(a[3], b[3]) - max(a[1], b[1]))


def rect_center(rect: tuple[float, float, float, float]) -> tuple[float, float]:
    return (rect[0] + rect[2]) / 2.0, (rect[1] + rect[3]) / 2.0


def rect_contains_point(rect: tuple[float, float, float, float], point: tuple[float, float]) -> bool:
    return rect[0] <= point[0] <= rect[2] and rect[1] <= point[1] <= rect[3]


def rect_gap(a: tuple[float, float, float, float], b: tuple[float, float, float, float]) -> tuple[float, float]:
    horizontal = max(0.0, max(a[0], b[0]) - min(a[2], b[2]))
    vertical = max(0.0, max(a[1], b[1]) - min(a[3], b[3]))
    return horizontal, vertical


def same_visibility(a: dict[str, Any], b: dict[str, Any]) -> bool:
    a_visibility = ((a.get("baseInfo") or {}).get("visibilityType") or "ALWAYS")
    b_visibility = ((b.get("baseInfo") or {}).get("visibilityType") or "ALWAYS")
    return a_visibility == b_visibility


def overlay_match_score(event_button: dict[str, Any], display_button: dict[str, Any], aspect: float) -> float:
    event_rect = fcl_button_rect(event_button, aspect)
    display_rect = fcl_button_rect(display_button, aspect)
    event_area = rect_area(event_rect)
    display_area = rect_area(display_rect)
    if event_area <= 0 or display_area <= 0:
        return 0.0

    overlap = rect_overlap_area(event_rect, display_rect)
    overlap_min = overlap / max(1.0, min(event_area, display_area))
    display_center_in_event = rect_contains_point(event_rect, rect_center(display_rect))
    event_center_in_display = rect_contains_point(display_rect, rect_center(event_rect))
    horizontal_gap, vertical_gap = rect_gap(event_rect, display_rect)
    event_w = event_rect[2] - event_rect[0]
    event_h = event_rect[3] - event_rect[1]
    display_w = display_rect[2] - display_rect[0]
    display_h = display_rect[3] - display_rect[1]
    vertical_overlap = max(0.0, min(event_rect[3], display_rect[3]) - max(event_rect[1], display_rect[1])) / max(1.0, min(event_h, display_h))
    horizontal_overlap = max(0.0, min(event_rect[2], display_rect[2]) - max(event_rect[0], display_rect[0])) / max(1.0, min(event_w, display_w))

    if overlap_min >= 0.25 or display_center_in_event or event_center_in_display:
        score = 100.0 + overlap_min * 100.0
        if display_center_in_event:
            score += 25.0
        if event_center_in_display:
            score += 10.0
        return score

    max_w = max(event_w, display_w)
    max_h = max(event_h, display_h)
    if vertical_overlap >= 0.65 and horizontal_gap <= max(250.0, max_w * 0.25):
        return 40.0 + vertical_overlap * 20.0 - horizontal_gap / max(1.0, max_w)
    if horizontal_overlap >= 0.65 and vertical_gap <= max(250.0, max_h * 0.25):
        return 40.0 + horizontal_overlap * 20.0 - vertical_gap / max(1.0, max_h)
    return 0.0


def match_fcl_overlay_buttons(buttons: list[dict[str, Any]], aspect: float) -> tuple[dict[int, int], set[int]]:
    display_indices = [
        index for index, button in enumerate(buttons)
        if not fcl_button_has_payload(button) and str(button.get("text") or "").strip()
    ]
    event_indices = [
        index for index, button in enumerate(buttons)
        if fcl_button_has_payload(button) and not str(button.get("text") or "").strip()
    ]
    matches: dict[int, int] = {}
    consumed: set[int] = set()

    for event_index in event_indices:
        event_button = buttons[event_index]
        best_index: int | None = None
        best_score = 0.0
        for display_index in display_indices:
            if display_index in consumed:
                continue
            display_button = buttons[display_index]
            if not same_visibility(event_button, display_button):
                continue
            score = overlay_match_score(event_button, display_button, aspect)
            if score > best_score:
                best_score = score
                best_index = display_index
        if best_index is not None and best_score >= 40.0:
            matches[event_index] = best_index
            consumed.add(best_index)
    return matches, consumed


def normalized_control_text(text: str) -> str:
    return "".join(re.findall(r"[A-Za-z0-9]+|[\u4e00-\u9fff]+", text or "")).casefold()


def normalized_control_words(text: str) -> set[str]:
    words: set[str] = set()
    for raw in re.findall(r"[A-Za-z0-9]+|[\u4e00-\u9fff]+", text or ""):
        word = raw.casefold()
        if len(word) < 2:
            continue
        words.add(word)
        if re.fullmatch(r"[\u4e00-\u9fff]+", word):
            for size in range(2, min(5, len(word)) + 1):
                for start in range(0, len(word) - size + 1):
                    words.add(word[start:start + size])
    return words


def fcl_button_grid_signature(button: dict[str, Any]) -> tuple[str, int, int, str]:
    base_info = button.get("baseInfo") or {}
    width = ((base_info.get("percentageWidth") or {}).get("size"))
    height = ((base_info.get("percentageHeight") or {}).get("size"))
    return (
        str(button.get("style") or ""),
        clamp_int(width, 0),
        clamp_int(height, 0),
        str(base_info.get("visibilityType") or "ALWAYS"),
    )


def inferable_grid_indices(buttons: list[dict[str, Any]]) -> set[int]:
    buckets: dict[tuple[str, int, int, str], list[int]] = {}
    for index, button in enumerate(buttons):
        if fcl_button_has_payload(button) or not str(button.get("text") or "").strip():
            continue
        signature = fcl_button_grid_signature(button)
        if signature[1] <= 0 or signature[2] <= 0:
            continue
        buckets.setdefault(signature, []).append(index)
    return {
        index
        for indices in buckets.values()
        if len(indices) >= 4
        for index in indices
    }


def infer_events_from_group_names(
    button: dict[str, Any],
    group_ids_by_name: dict[str, str],
    group_name: str,
) -> list[dict[str, str]]:
    text = str(button.get("text") or "")
    text_words = normalized_control_words(text)
    normalized_text = normalized_control_text(text)
    if not text_words and not normalized_text:
        return []

    matches: list[tuple[int, int, int, str]] = []
    group_prefix = normalized_control_text(group_name)
    for candidate_name, group_id in group_ids_by_name.items():
        if not group_id or candidate_name == group_name:
            continue
        candidate_words = normalized_control_words(candidate_name)
        normalized_candidate = normalized_control_text(candidate_name)
        if not candidate_words and not normalized_candidate:
            continue
        if (candidate_words and candidate_words <= text_words) or (normalized_candidate and normalized_candidate in normalized_text):
            prefix_score = 1 if group_prefix and normalized_candidate.startswith(group_prefix) else 0
            matches.append((prefix_score, len(normalized_candidate), len(candidate_name), group_id))

    matches.sort(reverse=True)
    return dedupe_events([{"type": "switch_layer", "key": group_id} for _, _, _, group_id in matches[:1]])


def infer_builtin_menu_events(button: dict[str, Any]) -> list[dict[str, str]]:
    text = normalized_control_text(str(button.get("text") or ""))
    if text in {"fcl菜单", "菜单"}:
        return [{"type": "launcher_event", "key": "launcher.event.switch_menu"}]
    if text in {"输入法", "输入文字"}:
        return [{"type": "launcher_event", "key": "launcher.event.switch_ime"}]
    if text == "社交":
        return [{"type": "key", "key": "GLFW_KEY_P"}]
    if text == "聊天":
        return [{"type": "key", "key": "GLFW_KEY_T"}]
    return []


def event_bind_targets(button: dict[str, Any]) -> set[str]:
    event_root = button.get("event") or {}
    targets: set[str] = set()
    for event_name in ("pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"):
        event = event_root.get(event_name) or {}
        targets.update(str(group_id) for group_id in event.get("bindViewGroup") or [])
    return targets


def layer_event_targets(group: dict[str, Any]) -> set[str]:
    targets: set[str] = set()
    for button in ((group.get("viewData") or {}).get("buttonList") or []):
        targets.update(event_bind_targets(button))
    return targets


def rect_distance(a: tuple[float, float, float, float], b: tuple[float, float, float, float]) -> float:
    horizontal, vertical = rect_gap(a, b)
    return math.hypot(horizontal, vertical)


def infer_reciprocal_layer_openers(data: dict[str, Any], aspect: float) -> dict[str, str]:
    groups = [group for group in data.get("viewGroups") or [] if isinstance(group, dict)]
    opener_scores: dict[str, tuple[float, str]] = {}

    group_index = {str(group.get("id") or ""): index for index, group in enumerate(groups)}
    group_ids_by_name = {
        str(group.get("name") or "Layer"): str(group.get("id") or "")
        for group in groups
        if group.get("id")
    }
    targets_by_group_id = {
        str(group.get("id") or ""): layer_event_targets(group)
        for group in groups
    }

    for source_group in groups:
        source_id = str(source_group.get("id") or "")
        source_buttons = ((source_group.get("viewData") or {}).get("buttonList") or [])
        candidates = [
            button for button in source_buttons
            if not fcl_button_has_payload(button)
            and str(button.get("text") or "").strip()
            and fcl_button_area_ratio(button, aspect) < 0.05
        ]
        if not candidates:
            continue

        for candidate in candidates:
            inferred_events = infer_events_from_group_names(candidate, group_ids_by_name, str(source_group.get("name") or ""))
            for event in inferred_events:
                target_id = event.get("key", "")
                if target_id and target_id != source_id:
                    index_distance = abs(group_index.get(target_id, 0) - group_index.get(source_id, 0))
                    button_id = str(candidate.get("id") or "")
                    score = index_distance * 10000.0 - 1.0
                    previous = opener_scores.get(button_id)
                    if previous is None or score < previous[0]:
                        opener_scores[button_id] = (score, target_id)

        for target_group in groups:
            target_id = str(target_group.get("id") or "")
            if not target_id or target_id == source_id:
                continue
            if target_group.get("visibility") != "INVISIBLE":
                continue
            source_words = normalized_control_words(str(source_group.get("name") or ""))
            target_words = normalized_control_words(str(target_group.get("name") or ""))
            source_targets = targets_by_group_id.get(source_id, set())
            if source_words and target_words and source_words & target_words and target_id not in source_targets:
                continue

            target_buttons = ((target_group.get("viewData") or {}).get("buttonList") or [])
            close_buttons = [
                button for button in target_buttons
                if source_id in event_bind_targets(button)
                and target_id in event_bind_targets(button)
                and 0.08 <= fcl_button_area_ratio(button, aspect) <= 0.50
            ]
            if not close_buttons:
                continue

            best_candidate: dict[str, Any] | None = None
            best_distance = float("inf")
            for candidate in candidates:
                candidate_rect = fcl_button_rect(candidate, aspect)
                distance = min(rect_distance(candidate_rect, fcl_button_rect(close_button, aspect)) for close_button in close_buttons)
                if distance < best_distance:
                    best_distance = distance
                    best_candidate = candidate
            if best_candidate is not None and best_distance <= 500.0:
                button_id = str(best_candidate.get("id") or "")
                index_distance = abs(group_index.get(target_id, 0) - group_index.get(source_id, 0))
                score = index_distance * 10000.0 + best_distance
                previous = opener_scores.get(button_id)
                if previous is None or score < previous[0]:
                    opener_scores[button_id] = (score, target_id)
    return {button_id: target_id for button_id, (_, target_id) in opener_scores.items()}


def overlay_shared_fields_zl(original: dict[str, Any], current: dict[str, Any], style_map: dict[str, str], absolute_as_percentage: bool = False, aspect: float = 16 / 9) -> dict[str, Any]:
    restored = deep_copy_json(original)
    base_info = current.get("baseInfo") or {}
    source_text = current.get("text")
    if isinstance(source_text, dict):
        restored["text"] = translatable(text_default(source_text), source_text)
    else:
        restored["text"] = translatable(str(source_text or ""), restored.get("text"))
    restored["uuid"] = str(current.get("id") or restored.get("uuid") or short_id() + short_id()[:6])
    restored["position"] = {
        "x": scale_position_to_zl(base_info.get("xPosition", 0)),
        "y": scale_position_to_zl(base_info.get("yPosition", 0)),
    }
    restored["buttonSize"] = make_zl_button_size(base_info, absolute_as_percentage=absolute_as_percentage, aspect=aspect)
    restored["buttonStyle"] = style_map.get(str(current.get("style") or "Default"), restored.get("buttonStyle"))
    restored["visibilityType"] = visibility_fcl_to_zl(base_info.get("visibilityType"))
    return restored


def fcl_button_to_zl_textbox(button: dict[str, Any], style_map: dict[str, str], absolute_as_percentage: bool = False, aspect: float = 16 / 9) -> dict[str, Any]:
    original = meta_original(button, "zl")
    if original is not None and isinstance(original, dict) and "clickEvents" not in original:
        restored = overlay_shared_fields_zl(original, button, style_map, absolute_as_percentage=absolute_as_percentage, aspect=aspect)
        return set_meta(restored, make_meta("fcl", "button", str(button.get("id") or restored.get("uuid") or short_id()), button, {"synthetic": True, "generatedFrom": "decorative-textbox"}))

    base_info = button.get("baseInfo") or {}
    text = str(button.get("text") or "")
    result = {
        "text": translatable(text),
        "uuid": str(button.get("id") or short_id() + short_id()[:6]),
        "position": {
            "x": scale_position_to_zl(base_info.get("xPosition", 0)),
            "y": scale_position_to_zl(base_info.get("yPosition", 0)),
        },
        "buttonSize": make_zl_button_size(base_info, absolute_as_percentage=absolute_as_percentage, aspect=aspect),
        "buttonStyle": style_map.get(str(button.get("style") or "Default")),
        "textAlignment": "Left",
        "textBold": False,
        "textItalic": False,
        "textUnderline": False,
        "visibilityType": visibility_fcl_to_zl(base_info.get("visibilityType")),
    }
    return set_meta(result, make_meta("fcl", "button", str(button.get("id") or result["uuid"]), button, {"synthetic": True, "generatedFrom": "decorative-textbox"}))


def fcl_button_to_zl(
    button: dict[str, Any],
    style_map: dict[str, str],
    strict: bool,
    group_name: str = "",
    group_ids_by_name: dict[str, str] | None = None,
    visual_button: dict[str, Any] | None = None,
    absolute_as_percentage: bool = False,
    aspect: float = 16 / 9,
) -> dict[str, Any]:
    original = meta_original(button, "zl")
    if original is not None and isinstance(original, dict) and "clickEvents" in original:
        restored = overlay_shared_fields_zl(original, visual_button or button, style_map, absolute_as_percentage=absolute_as_percentage, aspect=aspect)
        return set_meta(restored, make_meta("fcl", "button", str(button.get("id") or restored.get("uuid") or short_id()), button))

    visual_button = visual_button or button
    base_info = visual_button.get("baseInfo") or {}
    event_root = button.get("event") or {}
    text = str(visual_button.get("text") or button.get("text") or "")
    group_ids_by_name = group_ids_by_name or {}
    click_events: list[dict[str, str]] = []
    substitutions: list[dict[str, Any]] = []
    meaningful_events = [
        name for name in ("pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent")
        if fcl_event_has_payload(event_root.get(name) or {})
    ]
    for event_name in ("pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"):
        click_events.extend(
            fcl_event_to_zl_events(
                event_root.get(event_name) or {},
                strict,
                label=text,
                event_name=event_name,
                group_ids_by_name=group_ids_by_name,
                substitutions=substitutions,
            )
        )
    click_events = normalize_zl_click_events(click_events)

    press_event = event_root.get("pressEvent") or {}
    press_keycodes = fcl_keycode_list(press_event.get("outputKeycodes"))
    can_toggle = bool(press_event.get("autoKeep")) and bool(press_keycodes) and meaningful_events == ["pressEvent"]
    if event_root.get("Movable"):
        reason = "FCL movable button cannot be represented in ZL layout JSON; preserved in metadata"
        warn(f"{reason} on button {text!r}", strict, once=True)
        substitutions.append(substitution(
            {"type": "fcl_button_flag", "key": "Movable"},
            {"type": "metadata_only"},
            reason,
            category="events",
        ))
    if event_root.get("pointerFollow") and not any(code in FCL_MOUSE_REVERSE for code in [clamp_int(k) for k in press_keycodes]):
        reason = "FCL pointerFollow cannot be represented exactly in ZL; preserved in metadata"
        warn(f"{reason} on button {text!r}", strict, once=True)
        substitutions.append(substitution(
            {"type": "fcl_button_flag", "key": "pointerFollow"},
            {"type": "metadata_only"},
            reason,
            category="events",
        ))

    is_decorative = not click_events
    result = {
        "text": translatable(text),
        "uuid": str(button.get("id") or short_id() + short_id()[:6]),
        "position": {
            "x": scale_position_to_zl(base_info.get("xPosition", 0)),
            "y": scale_position_to_zl(base_info.get("yPosition", 0)),
        },
        "buttonSize": make_zl_button_size(base_info, absolute_as_percentage=absolute_as_percentage, aspect=aspect),
        "buttonStyle": style_map.get(str(visual_button.get("style") or button.get("style") or "Default")),
        "textAlignment": "Left",
        "textBold": False,
        "textItalic": False,
        "textUnderline": False,
        "visibilityType": visibility_fcl_to_zl(base_info.get("visibilityType")),
        "clickEvents": click_events,
        # ZL's hit-test path treats isPenetrable && isSwipple as a skip marker.
        # Use it for decorative FCL UI so large background/text controls render
        # without blocking interactive buttons layered under/around them.
        "isSwipple": is_decorative,
        "isPenetrable": is_decorative,
        "isToggleable": can_toggle,
    }
    mapping: dict[str, Any] | None = None
    if visual_button is not button:
        mapping = {
            "synthetic": True,
            "generatedFrom": "overlay-merge",
            "pairedVisualId": str(visual_button.get("id") or ""),
            "pairedEventId": str(button.get("id") or ""),
        }
    mapping = append_substitutions(mapping, substitutions)
    return set_meta(result, make_meta("fcl", "button", str(button.get("id") or result["uuid"]), button, mapping))


def fcl_direction_rect_to_zl_grid(
    direction: dict[str, Any],
    style: dict[str, Any],
    aspect: float,
    joined: bool = False
) -> tuple[int, int, int, int, int, int, float, float, str, dict[str, Any], float]:
    # FCL ControlDirection is rendered as one square parent view. BUTTON style then
    # places square child buttons inside it with Java integer division/truncation.
    base = direction.get("baseInfo") or {}
    button_style = (style or {}).get("buttonStyle") or {}
    absolute = base.get("sizeType") == "ABSOLUTE"

    if absolute:
        screen_h = 411.0
        screen_w = screen_h * max(0.1, clamp_float(aspect, 16 / 9))
        reference = "SCREEN_HEIGHT"
        reference_size = screen_h
        view_size = max(1, clamp_int(base.get("absoluteWidth"), 50))
    else:
        screen_h = 10000.0
        screen_w = screen_h * max(0.1, clamp_float(aspect, 16 / 9))
        pw = base.get("percentageWidth") or {}
        reference = pw.get("reference") or "SCREEN_WIDTH"
        reference_size = screen_h if reference == "SCREEN_HEIGHT" else screen_w
        view_size = max(1, int(reference_size * clamp_int(pw.get("size", 100)) / 1000.0))

    widget_x = int((screen_w - view_size) * clamp_int(base.get("xPosition", 0)) / 1000.0)
    widget_y = int((screen_h - view_size) * clamp_int(base.get("yPosition", 0)) / 1000.0)
    interval = max(0, min(499, clamp_int(button_style.get("interval", 50), 50)))
    child_size = max(1, int(view_size * (1000 - (2 * interval)) / 3000))
    if joined:
        # FCL ROCKER has one circular touch area instead of child buttons. When it is
        # approximated as ZL normal buttons, use the bundled ZL 8-way D-pad density:
        # screen-height sizing with a 1350/10000 minimum cell keeps the grid from
        # looking too loose/small on wide screens while preserving larger FCL inputs.
        if not absolute:
            reference = "SCREEN_HEIGHT"
            reference_size = screen_h
            child_size = max(child_size, int(screen_h * 1350 / 10000))
        gap = max(0, int(child_size * 3 * interval / max(1, 1000 - (2 * interval))))
        p0 = 0
        p1 = child_size + gap
        p2 = (child_size + gap) * 2
    else:
        p0 = 0
        p1 = child_size + int(view_size * interval / 1000)
        p2 = view_size - child_size

    child_percentage = max(100, min(10000, round(child_size / reference_size * 10000)))
    if absolute:
        button_size = {
            "type": "dp",
            "widthDp": clamp_zl_dp(child_size),
            "heightDp": clamp_zl_dp(child_size),
            "widthPercentage": child_percentage,
            "heightPercentage": child_percentage,
            "widthReference": "screen_height",
            "heightReference": "screen_height",
        }
    else:
        button_size = {
            "type": "percentage",
            "widthDp": 50.0,
            "heightDp": 50.0,
            "widthPercentage": child_percentage,
            "heightPercentage": child_percentage,
            "widthReference": fcl_ref_name_to_zl(reference),
            "heightReference": fcl_ref_name_to_zl(reference),
        }
    child_px = float(child_size)
    return widget_x, widget_y, child_percentage, p0, p1, p2, screen_w, screen_h, reference, button_size, child_px


def pixel_to_zl_position(pixel: int, screen: float, child: float) -> int:
    available = max(1.0, screen - child)
    return max(0, min(10000, round(pixel / available * 10000)))


def fcl_ref_name_to_zl(reference: str) -> str:
    return "screen_height" if reference == "SCREEN_HEIGHT" else "screen_width"


def direction_event_keycodes(event: dict[str, Any], name: str, default_keycode: int) -> list[Any]:
    value = event.get(name) if isinstance(event, dict) else None
    keycodes = fcl_keycode_list(value)
    return keycodes or [default_keycode]


def direction_to_zl_buttons(
    direction: dict[str, Any],
    style: dict[str, Any],
    style_uuid: str | None,
    strict: bool,
    aspect: float,
    joined: bool = False
) -> list[dict[str, Any]]:
    base = direction.get("baseInfo") or {}
    event = direction.get("event") or {}
    widget_x, widget_y, size, p0, p1, p2, screen_w, screen_h, reference, button_size, child_px = fcl_direction_rect_to_zl_grid(direction, style, aspect, joined=joined)
    up_keys = direction_event_keycodes(event, "upKeycode", GLFW_TO_FCL["GLFW_KEY_W"])
    down_keys = direction_event_keycodes(event, "downKeycode", GLFW_TO_FCL["GLFW_KEY_S"])
    left_keys = direction_event_keycodes(event, "leftKeycode", GLFW_TO_FCL["GLFW_KEY_A"])
    right_keys = direction_event_keycodes(event, "rightKeycode", GLFW_TO_FCL["GLFW_KEY_D"])
    entries = [
        ("◤", p0, p0, up_keys + left_keys, False),
        ("▲", p1, p0, up_keys, False),
        ("◥", p2, p0, up_keys + right_keys, False),
        ("◀", p0, p1, left_keys, False),
        ("", p1, p1, [], True),
        ("▶", p2, p1, right_keys, False),
        ("◣", p0, p2, down_keys + left_keys, False),
        ("▼", p1, p2, down_keys, False),
        ("◢", p2, p2, down_keys + right_keys, False),
    ]
    buttons = []
    for text, dx, dy, keycodes, is_center in entries:
        click_events = []
        substitutions: list[dict[str, Any]] = []
        for keycode in keycodes:
            converted = convert_key_to_zl(clamp_int(keycode), strict, label=text, substitutions=substitutions)
            if converted:
                etype, key = converted
                click_events.append({"type": etype, "key": key})
        if is_center:
            continue
        button_obj = {
            "text": translatable(text),
            "uuid": short_id() + short_id()[:6],
            "position": {
                "x": pixel_to_zl_position(round(widget_x + dx), screen_w, child_px),
                "y": pixel_to_zl_position(round(widget_y + dy), screen_h, child_px),
            },
            "buttonSize": deep_copy_json(button_size),
            "buttonStyle": style_uuid,
            "textAlignment": "Left",
            "textBold": False,
            "textItalic": False,
            "textUnderline": False,
            "visibilityType": visibility_fcl_to_zl(base.get("visibilityType")),
            "clickEvents": click_events,
            "isSwipple": True,
            "isPenetrable": False,
            "isToggleable": False,
        }
        buttons.append(set_meta(button_obj, make_meta(
            "fcl",
            "direction",
            str(direction.get("id") or ""),
            direction,
            append_substitutions({
                "synthetic": True,
                "generatedFrom": "direction-grid",
            }, substitutions),
        )))
    return buttons


def fcl_to_zl(data: dict[str, Any], include_directions: bool = False, strict: bool = False, aspect: float = 16 / 9, lossless: bool = False, absolute_as_percentage: bool = False) -> dict[str, Any]:
    include_directions = include_directions or lossless
    root_original = meta_original(data, "zl", "layout")
    styles, style_map = fcl_styles_to_zl(data.get("buttonStyles") or [default_fcl_style()])
    direction_styles = direction_style_map(data.get("directionStyles") or [default_fcl_direction_style()])
    default_style_uuid = next(iter(style_map.values()), None)
    layers = []
    special: dict[str, Any] = deep_copy_json((root_original or {}).get("special") or {})
    warned_joystick_settings = False
    group_ids_by_name = {
        str(group.get("name") or "Layer"): str(group.get("id") or "")
        for group in data.get("viewGroups") or []
        if isinstance(group, dict)
    }
    reciprocal_openers = infer_reciprocal_layer_openers(data, aspect)
    # FCL adds views in viewGroups order; later groups are on top in Android.
    # ZL hit-testing treats earlier layers as higher priority, so keep reverse order here.
    # Initial FCL parent visibility is preserved via each ZL layer's hide flag.
    for group in reversed(data.get("viewGroups") or []):
        layer_original = meta_original(group, "zl", "layer")
        view_data = group.get("viewData") or {}
        group_name = str(group.get("name") or "Layer")
        buttons = []
        text_boxes = []
        fcl_buttons = view_data.get("buttonList") or []
        overlay_matches, consumed_display_indices = match_fcl_overlay_buttons(fcl_buttons, aspect)
        grid_indices = inferable_grid_indices(fcl_buttons)
        for index, button in enumerate(fcl_buttons):
            if index in consumed_display_indices:
                continue
            has_payload = fcl_button_has_payload(button)
            if has_payload:
                visual_button = fcl_buttons[overlay_matches[index]] if index in overlay_matches else None
                converted_button = fcl_button_to_zl(
                    button,
                    style_map,
                    strict,
                    group_name=group_name,
                    group_ids_by_name=group_ids_by_name,
                    visual_button=visual_button,
                    absolute_as_percentage=absolute_as_percentage,
                    aspect=aspect,
                )
                if converted_button.get("clickEvents"):
                    buttons.append(converted_button)
                else:
                    text_boxes.append(fcl_button_to_zl_textbox(visual_button or button, style_map, absolute_as_percentage=absolute_as_percentage, aspect=aspect))
            else:
                opener_target = reciprocal_openers.get(str(button.get("id") or ""))
                inferred_events = [{"type": "switch_layer", "key": opener_target}] if opener_target else []
                if not inferred_events and index in grid_indices:
                    inferred_events = infer_events_from_group_names(button, group_ids_by_name, group_name)
                if not inferred_events and index in grid_indices:
                    inferred_events = infer_builtin_menu_events(button)
                if inferred_events:
                    inferred_button = fcl_button_to_zl(
                        button,
                        style_map,
                        strict,
                        group_name=group_name,
                        group_ids_by_name=group_ids_by_name,
                        absolute_as_percentage=absolute_as_percentage,
                        aspect=aspect,
                    )
                    inferred_button["clickEvents"] = inferred_events
                    inferred_button["isSwipple"] = False
                    inferred_button["isPenetrable"] = False
                    buttons.append(inferred_button)
                else:
                    buttons.append(fcl_button_to_zl(
                        button,
                        style_map,
                        strict,
                        group_name=group_name,
                        group_ids_by_name=group_ids_by_name,
                        absolute_as_percentage=absolute_as_percentage,
                        aspect=aspect,
                    ))
        directions = view_data.get("directionList") or []
        direction_buttons: list[dict[str, Any]] = []
        if directions and not include_directions:
            warn(f"skipped {len(directions)} FCL direction control(s) in group {group.get('name')!r}; use --include-directions to convert them", strict)
        if include_directions:
            for direction in directions:
                direction_style = resolve_direction_style(direction, direction_styles)
                is_rocker = direction_style.get("styleType") == "ROCKER"
                if is_rocker:
                    if "joystickStyle" not in special:
                        special["joystickStyle"] = fcl_rocker_style_to_zl_joystick(direction_style)
                    if not warned_joystick_settings:
                        warn("converted FCL ROCKER style to ZL special.joystickStyle and approximated rocker controls as 8-way button grid", strict)
                        warned_joystick_settings = True
                _SUBSTITUTION_COUNTS["directions"] += 1
                direction_buttons.extend(direction_to_zl_buttons(direction, direction_style, default_style_uuid, strict, aspect, joined=is_rocker))
        buttons.sort(key=lambda button: fcl_button_area_ratio({"baseInfo": {
            "xPosition": (button.get("position") or {}).get("x", 0) / 10,
            "yPosition": (button.get("position") or {}).get("y", 0) / 10,
            "sizeType": "PERCENTAGE",
            "percentageWidth": {"reference": "SCREEN_WIDTH", "size": ((button.get("buttonSize") or {}).get("widthPercentage", 0) / 10)},
            "percentageHeight": {"reference": "SCREEN_WIDTH", "size": ((button.get("buttonSize") or {}).get("heightPercentage", 0) / 10)},
        }}, aspect), reverse=True)
        buttons = direction_buttons + buttons

        layer_obj = deep_copy_json(layer_original) if layer_original is not None else {}
        layer_obj.update({
            "name": group_name,
            "uuid": str(group.get("id") or layer_obj.get("uuid") or short_id()),
            "hide": group.get("visibility") == "INVISIBLE",
            "hideWhenMouse": bool(layer_obj.get("hideWhenMouse", False)),
            "hideWhenGamepad": bool(layer_obj.get("hideWhenGamepad", False)),
            "hideWhenJoystick": bool(layer_obj.get("hideWhenJoystick", False)),
            "visibilityType": str(layer_obj.get("visibilityType") or "always"),
            "normalButtons": buttons,
            "textBoxes": text_boxes,
        })
        layers.append(set_meta(layer_obj, make_meta("fcl", "viewGroup", str(group.get("id") or layer_obj["uuid"]), group)))

    layer_ids = {layer["uuid"] for layer in layers}
    for layer in layers:
        for button in layer.get("normalButtons", []):
            button["clickEvents"] = [
                event for event in button.get("clickEvents", [])
                if event.get("type") not in {"switch_layer", "show_layer", "hide_layer"}
                or event.get("key") in layer_ids
            ]
            if not button["clickEvents"]:
                button["isSwipple"] = True
                button["isPenetrable"] = True
                button["isToggleable"] = False

    result = deep_copy_json(root_original) if root_original is not None else {}
    result.update({
        "info": {
            "name": translatable(str(data.get("name") or "Converted from FCL"), (result.get("info") or {}).get("name")),
            "author": translatable(str(data.get("author") or ""), (result.get("info") or {}).get("author")),
            "description": translatable(str(data.get("description") or ""), (result.get("info") or {}).get("description")),
            "versionCode": max(0, clamp_int(data.get("versionCode"), clamp_int((result.get("info") or {}).get("versionCode"), 1))),
            "versionName": str(data.get("version") or (result.get("info") or {}).get("versionName") or "1.0"),
        },
        "layers": layers,
        "styles": deep_copy_json((result.get("styles") if isinstance(result.get("styles"), list) and result.get("styles") else styles)),
        "editorVersion": clamp_int(result.get("editorVersion"), ZL_EDITOR_VERSION),
    })
    if special:
        result["special"] = special
    return set_meta(result, make_meta("fcl", "controller", str(data.get("id") or result.get("info", {}).get("name") or short_id()), data))


def detect_format(data: dict[str, Any]) -> str:
    if "layers" in data and "editorVersion" in data:
        return "zl"
    if "viewGroups" in data and "controllerVersion" in data:
        return "fcl"
    raise ValueError("cannot detect input format; use zl2fcl or fcl2zl explicitly")


def fcl_decorative_is_large_blank_blocker(button: dict[str, Any], aspect: float = 16 / 9) -> bool:
    if not fcl_button_is_decorative(button):
        return False
    if str(button.get("text") or "").strip():
        return False
    meta = get_meta(button) or {}
    original = meta.get("original") if isinstance(meta, dict) else None
    if isinstance(original, dict) and original.get("isPenetrable"):
        return False
    return fcl_button_area_ratio(button, aspect) >= 0.08


def make_fcl_usable(data: dict[str, Any], aspect: float = 16 / 9) -> dict[str, Any]:
    result = zl_to_fcl(data, strict=False)
    groups = result.get("viewGroups") or []
    removed_buttons: list[str] = []

    for group in groups:
        view_data = group.get("viewData")
        if not isinstance(view_data, dict):
            continue
        kept_buttons = []
        for button in view_data.get("buttonList") or []:
            if fcl_decorative_is_large_blank_blocker(button, aspect):
                removed_buttons.append(str(button.get("id") or ""))
                continue
            kept_buttons.append(button)
        view_data["buttonList"] = order_fcl_buttons_for_layer(kept_buttons)

    result["viewGroups"] = order_fcl_view_groups(groups)
    meta = get_meta(result) or {}
    mapping = meta.setdefault("mapping", {})
    mapping["profile"] = "structural-usable"
    mapping["removedLargeBlankDecorations"] = sorted(removed_buttons)
    result[META_KEY] = meta
    return result


def convert(mode: str, data: dict[str, Any], include_directions: bool, strict: bool, aspect: float, lossless: bool = False, absolute_as_percentage: bool = False, usable: bool = False) -> dict[str, Any]:
    if mode == "auto":
        detected = detect_format(data)
        mode = "zl2fcl" if detected == "zl" else "fcl2zl"
    if mode == "zl2fcl":
        return make_fcl_usable(data, aspect=aspect) if usable else zl_to_fcl(data, strict=strict)
    if mode == "fcl2zl":
        return normalize_zl_layout(fcl_to_zl(data, include_directions=include_directions, strict=strict, aspect=aspect, lossless=lossless, absolute_as_percentage=absolute_as_percentage))
    raise ValueError(f"unknown mode: {mode}")


def print_substitution_summary() -> None:
    total = sum(_SUBSTITUTION_COUNTS.values())
    if total == 0:
        return
    print(
        "conversion substitutions: "
        f"keys={_SUBSTITUTION_COUNTS['keys']}, "
        f"events={_SUBSTITUTION_COUNTS['events']}, "
        f"layers={_SUBSTITUTION_COUNTS['layers']}, "
        f"directions={_SUBSTITUTION_COUNTS['directions']}",
        file=sys.stderr,
    )


def parse_api_bool(value: Any, default: bool = False) -> bool:
    if value is None:
        return default
    if isinstance(value, bool):
        return value
    if isinstance(value, str):
        return value.strip().lower() in {"1", "true", "yes", "on"}
    return bool(value)


def strip_json_comments(text: str) -> str:
    result = []
    in_string = False
    escape = False
    index = 0
    while index < len(text):
        char = text[index]
        next_char = text[index + 1] if index + 1 < len(text) else ""
        if in_string:
            result.append(char)
            if escape:
                escape = False
            elif char == "\\":
                escape = True
            elif char == '"':
                in_string = False
            index += 1
        elif char == '"':
            in_string = True
            result.append(char)
            index += 1
        elif char == "/" and next_char == "/":
            index += 2
            while index < len(text) and text[index] not in "\r\n":
                index += 1
        elif char == "/" and next_char == "*":
            index += 2
            while index + 1 < len(text) and not (text[index] == "*" and text[index + 1] == "/"):
                index += 1
            index += 2
        else:
            result.append(char)
            index += 1
    return "".join(result)


def load_api_json(body: bytes) -> Any:
    text = body.decode("utf-8")
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        return json.loads(strip_json_comments(text))


def api_options_from_query(path: str) -> dict[str, Any]:
    query = parse_qs(urlparse(path).query)
    return {key: values[-1] for key, values in query.items() if values}


def handle_api_convert(payload: dict[str, Any]) -> dict[str, Any]:
    mode = str(payload.get("mode") or "auto")
    if mode not in {"auto", "zl2fcl", "fcl2zl"}:
        raise ValueError("mode must be one of: auto, zl2fcl, fcl2zl")
    if "data" not in payload or not isinstance(payload["data"], dict):
        raise ValueError("request JSON must contain object field 'data'")

    aspect = clamp_float(payload.get("aspect"), 16 / 9)
    if not math.isfinite(aspect) or aspect <= 0:
        raise ValueError("aspect must be a positive finite number")

    result = convert(
        mode,
        payload["data"],
        include_directions=parse_api_bool(payload.get("includeDirections")),
        strict=parse_api_bool(payload.get("strict")),
        aspect=aspect,
        lossless=parse_api_bool(payload.get("lossless")),
        absolute_as_percentage=parse_api_bool(payload.get("absoluteAsPercentage")),
        usable=parse_api_bool(payload.get("usable")),
    )
    if parse_api_bool(payload.get("stripMeta")):
        result = strip_converter_meta(result)
    return result


def handle_api_convert_file(data: dict[str, Any], options: dict[str, Any]) -> dict[str, Any]:
    mode = str(options.get("mode") or "auto")
    if mode not in {"auto", "zl2fcl", "fcl2zl"}:
        raise ValueError("mode must be one of: auto, zl2fcl, fcl2zl")

    aspect = clamp_float(options.get("aspect"), 16 / 9)
    if not math.isfinite(aspect) or aspect <= 0:
        raise ValueError("aspect must be a positive finite number")

    result = convert(
        mode,
        data,
        include_directions=parse_api_bool(options.get("includeDirections")),
        strict=parse_api_bool(options.get("strict")),
        aspect=aspect,
        lossless=parse_api_bool(options.get("lossless")),
        absolute_as_percentage=parse_api_bool(options.get("absoluteAsPercentage")),
        usable=parse_api_bool(options.get("usable")),
    )
    if parse_api_bool(options.get("stripMeta")):
        result = strip_converter_meta(result)
    return result


class ConverterApiHandler(BaseHTTPRequestHandler):
    server_version = "ControlConverterAPI/1.0"

    def log_message(self, format: str, *args: Any) -> None:
        print(f"{self.address_string()} - {format % args}", file=sys.stderr)

    def send_cors_headers(self) -> None:
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        self.send_header("Access-Control-Max-Age", "86400")

    def write_json(self, status: int, payload: dict[str, Any]) -> None:
        body = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_cors_headers()
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self) -> None:
        self.send_response(204)
        self.send_cors_headers()
        self.send_header("Content-Length", "0")
        self.end_headers()

    def do_GET(self) -> None:
        if self.path == "/health":
            self.write_json(200, {"ok": True})
        else:
            self.write_json(404, {"ok": False, "error": "not found"})

    def do_POST(self) -> None:
        parsed = urlparse(self.path)
        if parsed.path not in {"/convert", "/convert-file", "/convert-text"}:
            self.write_json(404, {"ok": False, "error": "not found"})
            return
        try:
            length = clamp_int(self.headers.get("Content-Length"), 0)
            body = self.rfile.read(length)
            if parsed.path in {"/convert-file", "/convert-text"}:
                data = load_api_json(body)
                if not isinstance(data, dict):
                    raise ValueError("uploaded layout must be a JSON object")
                result = handle_api_convert_file(data, api_options_from_query(self.path))
                self.write_json(200, result)
            else:
                payload = load_api_json(body)
                if not isinstance(payload, dict):
                    raise ValueError("request body must be a JSON object")
                result = handle_api_convert(payload)
                self.write_json(200, {"ok": True, "data": result})
        except Exception as exc:
            self.write_json(400, {"ok": False, "error": str(exc)})


def run_api_server(host: str, port: int) -> None:
    server = ThreadingHTTPServer((host, port), ConverterApiHandler)
    print(f"control converter api listening on http://{host}:{port}", file=sys.stderr)
    server.serve_forever()


def main() -> int:
    parser = argparse.ArgumentParser(description="Convert control JSON between Zalith Launcher 2 and FoldCraftLauncher.")
    parser.add_argument("mode", choices=["auto", "zl2fcl", "fcl2zl", "api"])
    parser.add_argument("input", type=Path, nargs="?")
    parser.add_argument("output", type=Path, nargs="?")
    parser.add_argument("--include-directions", action="store_true", help="approximate FCL direction controls as ZL button grids")
    parser.add_argument("--lossless", "--no-drop", action="store_true", help="substitute unsupported controls instead of dropping them; also converts FCL directions")
    parser.add_argument("--absolute-as-percentage", action="store_true", help="convert FCL absolute dp sizes to ZL percentage sizes using --aspect")
    parser.add_argument("--strict", action="store_true", help="fail instead of warning on unsupported fields/events")
    parser.add_argument("--compact", action="store_true", help="write compact JSON instead of pretty JSON")
    parser.add_argument("--strip-meta", action="store_true", help="remove _control_converter metadata from output JSON")
    parser.add_argument("--usable", action="store_true", help="ZL->FCL: structural safe mode; preserve groups/buttons and remove only large blank blockers")
    parser.add_argument("--aspect", type=float, default=16 / 9, help="screen width/height ratio used when approximating FCL direction controls")
    parser.add_argument("--host", default="127.0.0.1", help="API server host")
    parser.add_argument("--port", type=int, default=8000, help="API server port")
    args = parser.parse_args()

    if args.mode == "api":
        run_api_server(args.host, args.port)
        return 0

    if args.input is None or args.output is None:
        parser.error("input and output are required unless mode is api")

    if not math.isfinite(args.aspect) or args.aspect <= 0:
        parser.error("--aspect must be a positive finite number")

    with args.input.open("r", encoding="utf-8") as fh:
        source = json.load(fh)
    result = convert(args.mode, source, include_directions=args.include_directions, strict=args.strict, aspect=args.aspect, lossless=args.lossless, absolute_as_percentage=args.absolute_as_percentage, usable=args.usable)
    if args.strip_meta:
        result = strip_converter_meta(result)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", encoding="utf-8") as fh:
        if args.compact:
            json.dump(result, fh, ensure_ascii=False, separators=(",", ":"))
        else:
            json.dump(result, fh, ensure_ascii=False, indent=2)
            fh.write("\n")
    print_substitution_summary()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
