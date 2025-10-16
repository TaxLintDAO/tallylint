# #!/bin/bash

# # 获取备份文件的数量
# backup_files=$(ls backup_data_*.txt 2>/dev/null | wc -l)

# # 检查是否有备份文件
# if [ "$backup_files" -eq 0 ]; then
#     echo "未找到备份文件！请先运行备份脚本。"
#     exit 1
# fi

# # 循环恢复数据
# for ((i = 0; i < backup_files; i++)); do
#     backup_file="backup_data_$i.txt"
    
#     if [ -f "$backup_file" ]; then
#         echo "正在恢复数据，索引: $i"
        
#         # 使用 --argument-file 调用 restore_data
#         #线上恢复
#         # dfx canister call backend restore_data --argument-file "$backup_file" --ic
#         #线下测试
#         dfx canister call backend restore_data --argument-file "$backup_file"
        
#         echo "数据已恢复，索引: $i"
#     else
#         echo "备份文件 $backup_file 不存在，跳过。"
#     fi
# done

# echo "恢复完成！总共恢复了 $backup_files 个分块。"

#对以上脚本内容进行修改，之后只用进行一次数据恢复即可
#!/bin/bash

# 定义备份文件
backup_file="backup_data.txt"

# 检查备份文件是否存在
if [ ! -f "$backup_file" ]; then
    echo "未找到备份文件 $backup_file！请先运行备份脚本并确保数据已合并。"
    exit 1
fi

# 恢复数据
echo "正在恢复数据..."
# 线上恢复
# dfx canister call backend restore_data --argument-file "$backup_file" --ic
# 线下测试
dfx canister call backend restore_data --argument-file "$backup_file"

echo "数据恢复完成！"