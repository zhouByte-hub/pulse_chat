SET FOREIGN_KEY_CHECKS = 0;
SET NAMES utf8mb4;
-- contacts DDL
CREATE TABLE `contacts` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`user_id` BIGINT UNSIGNED NOT NULL,
`contact_id` BIGINT UNSIGNED NOT NULL,
`group_name` VARCHAR(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '好友',
`created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
INDEX `idx_contact_id`(`contact_id` ASC) USING BTREE,
INDEX `idx_user_id`(`user_id` ASC) USING BTREE,
UNIQUE INDEX `uk_user_contact`(`user_id` ASC,`contact_id` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci AUTO_INCREMENT = 64 ROW_FORMAT = Dynamic;
-- conversations DDL
CREATE TABLE `conversations` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`user_id` BIGINT UNSIGNED NOT NULL,
`target_id` BIGINT UNSIGNED NOT NULL,
`target_type` ENUM("user","group") CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL DEFAULT 'user',
`last_message_id` BIGINT UNSIGNED NULL,
`unread_count` INT NULL DEFAULT 0,
`is_pinned` TINYINT(1) NULL DEFAULT 0,
`last_activity_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
`created_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
`updated_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP(0),
INDEX `idx_last_activity`(`last_activity_at` ASC) USING BTREE,
INDEX `idx_user_id`(`user_id` ASC) USING BTREE,
UNIQUE INDEX `uk_user_target`(`user_id` ASC,`target_id` ASC,`target_type` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci ROW_FORMAT = Dynamic;
-- group_members DDL
CREATE TABLE `group_members` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`group_id` BIGINT UNSIGNED NOT NULL,
`user_id` BIGINT UNSIGNED NOT NULL,
`role` ENUM("owner","admin","member") CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL DEFAULT 'member',
`joined_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
INDEX `idx_group_id`(`group_id` ASC) USING BTREE,
INDEX `idx_user_id`(`user_id` ASC) USING BTREE,
UNIQUE INDEX `uk_group_user`(`group_id` ASC,`user_id` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci ROW_FORMAT = Dynamic;
-- group_messages DDL
CREATE TABLE `group_messages` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`group_id` BIGINT UNSIGNED NOT NULL,
`sender_id` BIGINT UNSIGNED NOT NULL,
`content` TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
`message_type` ENUM("text","image","file","voice") CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL DEFAULT 'text',
`file_url` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL,
`file_size` INT NULL,
`created_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
INDEX `idx_created_at`(`created_at` ASC) USING BTREE,
INDEX `idx_group_id`(`group_id` ASC) USING BTREE,
INDEX `idx_sender_id`(`sender_id` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci ROW_FORMAT = Dynamic;
-- messages DDL
CREATE TABLE `messages` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`sender_id` BIGINT UNSIGNED NOT NULL,
`receiver_id` BIGINT UNSIGNED NOT NULL,
`content` TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
`message_type` ENUM("text","image","file","voice") CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'text',
`file_url` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL,
`file_size` INT NULL,
`is_read` TINYINT(1) NOT NULL DEFAULT 0,
`created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
INDEX `idx_created_at`(`created_at` ASC) USING BTREE,
INDEX `idx_receiver_sender`(`receiver_id` ASC,`sender_id` ASC) USING BTREE,
INDEX `idx_sender_receiver`(`sender_id` ASC,`receiver_id` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci AUTO_INCREMENT = 1 ROW_FORMAT = Dynamic;
-- pulse_group DDL
CREATE TABLE `pulse_group` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`name` VARCHAR(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
`description` TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL,
`avatar` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL,
`owner_id` BIGINT UNSIGNED NOT NULL,
`is_public` TINYINT(1) NULL DEFAULT 0,
`max_members` INT NULL DEFAULT 500,
`created_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
`updated_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP(0),
INDEX `idx_is_public`(`is_public` ASC) USING BTREE,
INDEX `idx_owner_id`(`owner_id` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci AUTO_INCREMENT = 1 ROW_FORMAT = Dynamic;
-- users DDL
CREATE TABLE `users` (`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`username` VARCHAR(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
`email` VARCHAR(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
`password_hash` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
`nickname` VARCHAR(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL,
`avatar` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL,
`phone` VARCHAR(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL Comment "手机号",
`description` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL Comment "个人简介",
`address` VARCHAR(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL Comment "所在地",
`status` ENUM("online","offline","away","busy") CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL DEFAULT 'offline',
`created_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
`updated_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP(0),
UNIQUE INDEX `email`(`email` ASC) USING BTREE,
INDEX `idx_email`(`email` ASC) USING BTREE,
INDEX `idx_status`(`status` ASC) USING BTREE,
INDEX `idx_username`(`username` ASC) USING BTREE,
UNIQUE INDEX `username`(`username` ASC) USING BTREE,
PRIMARY KEY (`id`)) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci AUTO_INCREMENT = 22 ROW_FORMAT = Dynamic;
-- contacts Indexes
ALTER TABLE `contacts` 
 ADD CONSTRAINT `contacts_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION,
ADD CONSTRAINT `contacts_ibfk_2` FOREIGN KEY (`contact_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION;
-- conversations Indexes
ALTER TABLE `conversations` 
 ADD CONSTRAINT `conversations_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION;
-- group_members Indexes
ALTER TABLE `group_members` 
 ADD CONSTRAINT `group_members_ibfk_1` FOREIGN KEY (`group_id`) REFERENCES `pulse_group` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION,
ADD CONSTRAINT `group_members_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION;
-- group_messages Indexes
ALTER TABLE `group_messages` 
 ADD CONSTRAINT `group_messages_ibfk_2` FOREIGN KEY (`sender_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION,
ADD CONSTRAINT `group_messages_ibfk_1` FOREIGN KEY (`group_id`) REFERENCES `pulse_group` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION;
-- messages Indexes
ALTER TABLE `messages` 
 ADD CONSTRAINT `messages_ibfk_1` FOREIGN KEY (`sender_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION,
ADD CONSTRAINT `messages_ibfk_2` FOREIGN KEY (`receiver_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION;
-- pulse_group Indexes
ALTER TABLE `pulse_group` 
 ADD CONSTRAINT `pulse_group_ibfk_1` FOREIGN KEY (`owner_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION;
-- contacts DML
INSERT INTO `contacts` (`id`,`user_id`,`contact_id`,`group_name`,`created_at`) VALUES (1,1,2,'同事','2024-01-16 09:00:00'),(2,1,3,'朋友','2024-01-17 10:30:00'),(3,1,4,'兴趣','2024-01-18 14:20:00'),(4,1,5,'工作','2024-01-19 16:45:00'),(5,2,1,'项目成员','2024-02-21 10:15:00'),(6,2,3,'团队','2024-02-22 11:00:00'),(7,2,6,'户外','2024-02-23 15:30:00'),(8,2,7,'设计','2024-02-24 17:20:00'),(9,3,1,'大学同学','2024-03-11 12:00:00'),(10,3,2,'同事','2024-03-12 13:15:00'),(11,3,8,'技术交流','2024-03-13 16:40:00'),(12,3,9,'旅行','2024-03-14 18:00:00'),(13,4,1,'朋友','2024-04-06 09:30:00'),(14,4,5,'安全组','2024-04-07 11:45:00'),(15,4,10,'开源社区','2024-04-08 14:10:00'),(16,5,2,'安全团队','2024-05-19 10:00:00'),(17,5,4,'同事','2024-05-20 12:30:00'),(18,5,1,'前同事','2024-05-21 15:00:00'),(19,6,2,'徒步伙伴','2024-06-23 09:15:00'),(20,6,7,'摄影圈','2024-06-24 13:20:00'),(21,6,8,'数据分析','2024-06-25 16:50:00'),(22,7,3,'开发协作','2024-07-09 10:45:00'),(23,7,6,'户外','2024-07-10 14:00:00'),(24,7,9,'旅行','2024-07-11 17:30:00'),(25,8,3,'技术','2024-08-15 11:20:00'),(26,8,6,'数据采集','2024-08-16 14:10:00'),(27,8,10,'机器学习','2024-08-17 16:00:00'),(28,9,1,'阅读','2024-09-02 10:00:00'),(29,9,7,'美学','2024-09-03 13:30:00'),(30,9,11,'产品设计','2024-09-04 15:45:00'),(31,10,4,'音乐','2024-10-12 09:50:00'),(32,10,8,'研究','2024-10-13 12:15:00'),(33,10,12,'全栈','2024-10-14 15:20:00'),(34,11,9,'产品经理','2024-11-04 10:10:00'),(35,11,13,'设计','2024-11-05 13:40:00'),(36,11,14,'技术','2024-11-06 16:20:00'),(37,12,10,'开源','2024-12-20 11:00:00'),(38,12,13,'UI/UX','2024-12-21 14:30:00'),(39,12,14,'人工智能','2024-12-22 17:15:00'),(40,13,7,'设计交流','2025-01-26 09:20:00'),(41,13,11,'产品协作','2025-01-27 12:50:00'),(42,13,15,'摄影','2025-01-28 15:30:00'),(43,14,8,'学术','2025-02-15 10:00:00'),(44,14,12,'开发','2025-02-16 13:15:00'),(45,14,16,'后端','2025-02-17 16:40:00'),(46,15,9,'旅行','2025-03-09 09:45:00'),(47,15,13,'视觉','2025-03-10 12:20:00'),(48,15,17,'区块链','2025-03-11 15:00:00'),(49,16,14,'技术','2025-04-18 10:30:00'),(50,16,18,'移动端','2025-04-19 13:45:00'),(51,16,19,'数据分析','2025-04-20 16:10:00'),(52,17,15,'NFT','2025-05-23 11:15:00'),(53,17,19,'数据','2025-05-24 14:20:00'),(54,17,20,'运维','2025-05-25 17:00:00'),(55,18,12,'开发','2025-06-11 10:00:00'),(56,18,16,'后端','2025-06-12 13:30:00'),(57,18,20,'DevOps','2025-06-13 16:20:00'),(58,19,8,'机器学习','2025-07-06 09:15:00'),(59,19,16,'数据库','2025-07-07 12:40:00'),(60,19,17,'区块链','2025-07-08 15:20:00'),(61,20,12,'开发','2025-08-02 08:30:00'),(62,20,16,'运维','2025-08-03 11:15:00'),(63,20,18,'移动端','2025-08-04 14:00:00');
-- messages DML
INSERT INTO `messages` (`id`,`sender_id`,`receiver_id`,`content`,`message_type`,`file_url`,`file_size`,`is_read`,`created_at`) VALUES (1,1,2,'123123','text',NULL,NULL,1,'2025-09-05 15:42:23'),(2,1,2,'hello world','text',NULL,NULL,1,'2025-09-05 15:43:15'),(3,1,2,'假的假的假的假的就','text',NULL,NULL,1,'2025-09-05 15:56:21'),(4,1,2,'1111','text',NULL,NULL,0,'2025-09-05 16:05:44'),(5,1,3,'123123','text',NULL,NULL,0,'2025-09-05 16:11:52');
-- users DML
INSERT INTO `users` (`id`,`username`,`email`,`password_hash`,`nickname`,`avatar`,`phone`,`description`,`address`,`status`,`created_at`,`updated_at`) VALUES (1,'time_travel','123456789@qq.com','3337D1778762F1D0D2EE08B87461844B','胖子','https://miaobi-lite.bj.bcebos.com/miaobi/5mao/b%276Jyh56yU5bCP5paw5oOF5L6j5aS05YOPXzE3Mjg5NDgyODguMjQ1MjcyMg%3D%3D%27/0.png','13245678911','一个励志要吃遍天下的胖子','湖南长沙','online','2025-09-01 15:49:42','2025-09-05 16:36:06'),(2,'alice_wonder','alice@example.com','3337D1778762F1D0D2EE08B87461844B','Alice','https://q9.itc.cn/q_70/images03/20241013/d770472d4906402c866b9c71a0c9927c.jpeg','13800138001','热爱阅读与冒险','北京','online','2024-01-15 08:30:00','2025-09-05 16:40:52'),(3,'bob_builder','bob@example.com','3337D1778762F1D0D2EE08B87461844B','Bob','https://q4.itc.cn/q_70/images03/20241013/f4b3e72c1e264d429aaf802f8b2eac96.jpeg','13800138002','永远在建造更好的东西','上海','busy','2024-02-20 09:15:00','2025-09-05 16:41:50'),(4,'carol_smith','carol@example.com','3337D1778762F1D0D2EE08B87461844B','Carol','https://q8.itc.cn/q_70/images03/20241030/7e4e379bf5b84b6c9fb4a9fec3102d5d.jpeg','13800138003','前端开发爱好者','深圳','away','2024-03-10 14:22:00','2025-09-05 16:42:19'),(5,'david_lee','david@example.com','3337D1778762F1D0D2EE08B87461844B','David','https://c-ssl.dtstatic.com/uploads/blog/202308/30/AvSZw51zfwVAL2p.thumb.1000_0.jpg','13800138004','音乐与咖啡是我的日常','杭州','online','2024-04-05 11:05:00','2025-09-05 16:48:46'),(6,'eve_charlie','eve@example.com','3337D1778762F1D0D2EE08B87461844B','Eve','https://c-ssl.dtstatic.com/uploads/blog/202408/14/9WSP7q6eh8wwMP6.thumb.1000_0.jpg','13800138005','网络安全研究员','广州','offline','2024-05-18 16:40:00','2025-09-05 16:44:54'),(7,'frank_wong','frank@example.com','3337D1778762F1D0D2EE08B87461844B','Frank','https://c-ssl.dtstatic.com/uploads/blog/202308/30/AvSZw51zfwVAL2p.thumb.1000_0.jpg','13800138006','喜欢徒步和摄影','成都','online','2024-06-22 10:30:00','2025-09-05 16:48:46'),(8,'grace_kim','grace@example.com','3337D1778762F1D0D2EE08B87461844B','Grace','https://c-ssl.dtstatic.com/uploads/blog/202209/10/20220910103801_c30c1.thumb.1000_0.jpg','13800138007','UI/UX 设计师','南京','busy','2024-07-08 13:15:00','2025-09-05 16:48:20'),(9,'henry_taylor','henry@example.com','3337D1778762F1D0D2EE08B87461844B','Henry','https://pic.rmb.bdstatic.com/bjh/bb8cc27b052/241128/91c4d5570217d40718ceca4e90606365.jpeg','13800138008','数据科学家','武汉','away','2024-08-14 08:45:00','2025-09-05 16:48:20'),(10,'irene_chen','irene@example.com','3337D1778762F1D0D2EE08B87461844B','Irene','https://c-ssl.dtstatic.com/uploads/blog/202211/04/20221104153701_17ed1.thumb.1000_0.jpg','13800138009','热爱旅行与美食','西安','online','2024-09-01 17:20:00','2025-09-05 16:48:20'),(11,'jack_miller','jack@example.com','3337D1778762F1D0D2EE08B87461844B','Jack','https://c-ssl.dtstatic.com/uploads/blog/202404/02/0GS9xeqnc0GGXMO.thumb.1000_0.jpg','13800138010','开源项目贡献者','天津','busy','2024-10-11 12:00:00','2025-09-05 16:48:20'),(12,'kate_white','kate@example.com','3337D1778762F1D0D2EE08B87461844B','Kate','https://c-ssl.dtstatic.com/uploads/blog/202408/30/0GSn5baVU0ZBwjB.thumb.1000_0.jpg','13800138011','产品经理','重庆','offline','2024-11-03 09:30:00','2025-09-05 16:48:21'),(13,'leo_zhang','leo@example.com','3337D1778762F1D0D2EE08B87461844B','Leo','https://b0.bdstatic.com/ugc/FCpb93A_Bjr0gN0gUvxoOQ6de75f416f775518fd148577647569ce.jpg','13800138012','全栈工程师','苏州','online','2024-12-19 15:10:00','2025-09-05 16:48:21'),(14,'mia_liu','mia@example.com','3337D1778762F1D0D2EE08B87461844B','Mia','https://c-ssl.dtstatic.com/uploads/blog/202312/24/5zS3lo79cOY6g12.thumb.1000_0.jpg','13800138013','平面设计师','青岛','away','2025-01-25 10:40:00','2025-09-05 16:48:21'),(15,'nathan_yu','nathan@example.com','3337D1778762F1D0D2EE08B87461844B','Nathan','https://c-ssl.dtstatic.com/uploads/blog/202308/30/AvSZw51zfwVAL2p.thumb.1000_0.jpg','13800138014','AI 研究员','大连','busy','2025-02-14 14:00:00','2025-09-05 16:48:21'),(16,'olivia_wu','olivia@example.com','3337D1778762F1D0D2EE08B87461844B','Olivia','https://ww1.sinaimg.cn/mw690/005J4OU5ly1huxb0v5u9uj30u00u0mzv.jpg','13800138015','产品经理 & 摄影师','厦门','online','2025-03-08 08:20:00','2025-09-05 16:44:55'),(17,'peter_lin','peter@example.com','3337D1778762F1D0D2EE08B87461844B','Peter','https://c-ssl.dtstatic.com/uploads/blog/202408/14/9WSP7q6eh8wwMP6.thumb.1000_0.jpg','13800138016','后端开发','长沙','offline','2025-04-17 16:50:00','2025-09-05 16:44:55'),(18,'qin_zhao','qin@example.com','3337D1778762F1D0D2EE08B87461844B','Qin','https://ww1.sinaimg.cn/mw690/006i0nC8ly1hss519035rj32yw2ywhdt.jpg','13800138017','区块链开发者','合肥','busy','2025-05-22 11:25:00','2025-09-05 16:48:21'),(19,'ryan_xu','ryan@example.com','3337D1778762F1D0D2EE08B87461844B','Ryan','https://c-ssl.dtstatic.com/uploads/blog/202209/10/20220910103801_c30c1.thumb.1000_0.jpg','13800138018','移动应用开发','福州','away','2025-06-10 13:40:00','2025-09-05 16:48:21'),(20,'sophia_huang','sophia@example.com','3337D1778762F1D0D2EE08B87461844B','Sophia','https://pic.rmb.bdstatic.com/bjh/bb8cc27b052/241128/91c4d5570217d40718ceca4e90606365.jpeg','13800138019','数据分析师','郑州','online','2025-07-05 09:10:00','2025-09-05 16:48:22'),(21,'tom_li','tom@example.com','3337D1778762F1D0D2EE08B87461844B','Tom','https://c-ssl.dtstatic.com/uploads/blog/202211/04/20221104153701_17ed1.thumb.1000_0.jpg','13800138020','DevOps 工程师','哈尔滨','busy','2025-08-01 10:05:00','2025-09-05 16:48:22');
SET FOREIGN_KEY_CHECKS = 1;
