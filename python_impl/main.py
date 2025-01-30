from segment_tree import SegmentTree

def main():
    st = SegmentTree([12,4,4,3,4,23,4,2])
    st.update(0, 1)
    print(st.query(0, 0))
    st.update(1, 2)
    
    print(st.query(1, 1))
    st.update(2, 3)
    
    print(st.query(2, 2))
    st.update(3, 4)
    
    print(st.query(3, 3))
    st.update(4, 5)
    print(st.query(0, 4))
    

if __name__ == '__main__':
    main()