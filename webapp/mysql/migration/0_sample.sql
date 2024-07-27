-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

CREATE FULLTEXT INDEX idx_area_name_ft ON areas(name) WITH PARSER ngram;

CREATE FULLTEXT INDEX idx_user_username_ft ON users(username) WITH PARSER ngram;
CREATE FULLTEXT INDEX idx_user_role_ft ON users(role) WITH PARSER ngram;

CREATE INDEX idx_session_user_id ON sessions(user_id);
CREATE INDEX idx_session_token ON sessions(session_token);

CREATE INDEX idx_dispatcher_user_id ON dispatchers(user_id);
CREATE INDEX idx_dispatcher_area_id ON dispatchers(area_id);

CREATE INDEX idx_tow_truck_driver_id ON tow_trucks(driver_id);
CREATE INDEX idx_tow_truck_status ON tow_trucks(status);
CREATE INDEX idx_tow_truck_area_id ON tow_trucks(area_id);

CREATE FULLTEXT INDEX idx_node_name_ft ON nodes(name) WITH PARSER ngram;
CREATE INDEX idx_node_area_id ON nodes(area_id);

CREATE INDEX idx_edges_node_a_id ON edges(node_a_id);
CREATE INDEX idx_edges_node_b_id ON edges(node_b_id);

CREATE INDEX idx_location_tow_truck_id ON locations(tow_truck_id);
CREATE INDEX idx_location_node_id ON locations(node_id);
CREATE INDEX idx_location_timestamp ON locations(timestamp);

CREATE INDEX idx_order_client_id ON orders(client_id);
CREATE INDEX idx_order_dispatcher_id ON orders(dispatcher_id);
CREATE INDEX idx_order_tow_truck_id ON orders(tow_truck_id);
CREATE INDEX idx_order_status ON orders(status);
CREATE INDEX idx_order_node_id ON orders(node_id);
CREATE INDEX idx_order_order_time ON orders(order_time);
CREATE INDEX idx_order_completed_time ON orders(completed_time);

CREATE INDEX idx_completed_order_order_id ON completed_orders(order_id);
CREATE INDEX idx_completed_order_tow_truck_id ON completed_orders(tow_truck_id);
CREATE INDEX idx_completed_order_completed_time ON completed_orders(completed_time);