#ifndef __REGISTER_H_
#define __REGISTER_H_

template <typename T>
struct ReadOnly{
private:
    T _r;
public:
    inline T read(void){return _r;};
    inline T read_mask(T mask){return _r & mask;};
    /* read bit */
    inline bool read_bit(int bitIdx){return (_r >> bitIdx)&0x01;}
	/* read number of bit */
    inline T read_bits(int start, int width){return (T)(_r >> start & ((0x01 << width) - 1));}
};

template <typename T>
struct WriteOnly{
private:
    T _r;
public:
    /* write register */
    inline void write(T var){_r = var;}
	/* set mask register to 0 */
    inline void clear_mask(T mask){_r &= ~mask;}
	inline void clear_bits(T mask, int start, int width){
        T vmask = ((0x01 << width) - 1) << start;
        mask <<= start;
        mask &= vmask;
        _r &= ~mask;
    }
	/* set mask register to 1 */
	inline void set_mask(T mask){_r |= mask;}
    inline void set_bits(T mask, int start, int width){
        T vmask = ((0x01 << width) - 1) << start;
        mask <<= start;
        mask &= vmask;
        _r |= mask;
    }
    inline void write_mask(T v,T mask){
        v &= mask;
        _r &= ~mask;
        _r |= v;
    }
     inline void write_bits(T v, int start, int width){
        T mask;
        mask = ((0x01 << width) - 1) << start;
        v <<= start;
        v &= mask;
        _r &= ~(mask);
        _r |= v;
    }
};

template <typename T>
struct ReadWrite{
private:
    T _r;
public:
    inline T read(void){return _r;};
    inline T read_mask(T mask){return _r & mask;};
    /* read bit */
    inline bool read_bit(int bitIdx){return (_r >> bitIdx)&0x01;}
	/* read number of bit */
    inline T read_bits(int start, int width){return (T)(_r >> start & ((0x01 << width) - 1));}
    /* write register */
    inline void write(T var){_r = var;}
	/* set mask register to 0 */
    inline void clear_mask(T mask){_r &= (~mask);}
    inline void clear_bits(T mask, int start, int width){
        T vmask = ((0x01 << width) - 1) << start;
        mask <<= start;
        mask &= vmask;
        _r &= ~mask;
    }
	/* set mask register to 1 */
	inline void set_mask(T mask){_r |= mask;}
    inline void set_bits(T mask, int start, int width){
        T vmask = ((0x01 << width) - 1) << start;
        mask <<= start;
        mask &= vmask;
        _r |= mask;
    }
	inline void write_mask(T v,T mask){
        v &= mask;
        _r &= ~mask;
        _r |= v;
    }
    inline void write_bits(T v, int start, int width){
        T mask;
        mask = ((0x01 << width) - 1) << start;
        v <<= start;
        v &= mask;
        _r &= ~(mask);
        _r |= v;
    }
};


#endif