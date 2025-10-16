# #!/bin/bash

# # 初始化变量
# index=0
# backup_file="backup_chunk_$index.txt"

# # 第一次备份
# echo "开始备份，索引: $index"
# #线上测试
# dfx canister call backend backup_data "(0)" --ic > "$backup_file"
# #线下备份
# # dfx canister call backend backup_data "(0)" > "$backup_file"

# # 解析第一次备份的结果
# result=$(cat "$backup_file")

# # 提取 total 值
# total=$(echo "$result" | grep -oP 'total = \K\d+')

# # 提取 blob 部分（包括 "blob" 关键字）
# blob_data=$(echo "$result" | grep -oP 'blob "[^"]+"')

# # 去掉最后的分号
# blob_data=${blob_data%;}

# # 保存备份数据到文件
# echo "$blob_data" > "backup_data_$index.txt"
# echo "备份数据已保存到 backup_data_$index.txt"

# # 循环备份剩余数据
# for ((i = 1; i < total; i++)); do
#     index=$i
#     backup_file="backup_chunk_$index.txt"
    
#     echo "开始备份，索引: $index"
#     #线上备份
#     dfx canister call backend backup_data "($index)" --ic > "$backup_file"
#     #线下测试
#     # dfx canister call backend backup_data "($index)" > "$backup_file"
    
#     # 解析备份结果
#     result=$(cat "$backup_file")
#     blob_data=$(echo "$result" | grep -oP 'blob "[^"]+"')
    
#     # 去掉最后的分号
#     blob_data=${blob_data%;}
    
#     # 保存备份数据到文件
#     echo "$blob_data" > "backup_data_$index.txt"
#     echo "备份数据已保存到 backup_data_$index.txt"
# done

# echo "备份完成！总共备份了 $total 个分块。"

#以下脚本内容将原先的备份数据合并到一个文件中
#!/bin/bash

# 初始化变量
index=0
backup_file="backup_chunk_$index.txt"
output_file="backup_data.txt"

# 清空或创建最终的备份文件
> "$output_file"

# 第一次备份
echo "开始备份，索引: $index"
# 线上测试
dfx canister call backend backup_data "(0)" --ic > "$backup_file"
# 线下备份
# dfx canister call backend backup_data "(0)" > "$backup_file"

# 解析第一次备份的结果
result=$(cat "$backup_file")

# 提取 total 值
total=$(echo "$result" | grep -oP 'total = \K\d+')

# 提取 blob 数据内容（去掉 "blob" 和引号）
blob_data=$(echo "$result" | grep -oP 'blob "\K[^"]+')

# 将 blob 数据写入最终的备份文件
echo -n "$blob_data" > "$output_file"
echo "备份数据已保存到 backup_data_$index.txt"

# 循环备份剩余数据
for ((i = 1; i < total; i++)); do
    index=$i
    backup_file="backup_chunk_$index.txt"
    
    echo "开始备份，索引: $index"
    # 线上备份
    # dfx canister call backend backup_data "($index)" --ic > "$backup_file"
    # 线下测试
    dfx canister call backend backup_data "($index)" > "$backup_file"
    
    # 解析备份结果
    result=$(cat "$backup_file")
    blob_data=$(echo "$result" | grep -oP 'blob "\K[^"]+')
    
    # 将 blob 数据追加到最终的备份文件
    echo -n "$blob_data" >> "$output_file"
    echo "备份数据已保存到 backup_data_$index.txt"
done

# 将最终的内容包装成 blob 格式
final_content=$(cat "$output_file")
echo "blob \"$final_content\"" > "$output_file"

echo "备份完成！总共备份了 $total 个分块。"
echo "所有分块数据已合并到 $output_file"